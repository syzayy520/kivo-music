# C++ Handoff Contract Assets

**Date:** 2026-06-10  
**Source Branch:** `kivo-audio-native-decode-pipeline-p0-009`  
**Source HEAD:** `b4e81a3bd760a8f46d1105070af68741bfcc9a76`

This document captures the architectural specifications, boundary contracts, testing philosophies, and design patterns from the Rust audio backend that should be inherited by the C++ audio core project.

---

## 1. DeviceBufferWriter Contract Concept

### Core Trait/Interface
```cpp
class IDeviceBufferWriter {
public:
    virtual ~IDeviceBufferWriter() = default;
    
    virtual WriteResult write(const WriteRequest& request) = 0;
    virtual WriteResult flush() = 0;
    virtual WriteResult reset() = 0;
    virtual WriterState snapshot() const = 0;
    virtual WriterCursor cursor() const = 0;
    virtual WriteResult close() = 0;
    virtual RuntimeMode runtime_mode() const = 0;
};
```

### Design Principles
- **Single responsibility:** Each method does one thing
- **Pure data types:** All inputs/outputs are value types, no pointers
- **Error handling:** All errors returned via `WriteResult`/`WriteError`, not exceptions
- **Snapshot pattern:** `snapshot()` returns immutable state copy
- **Cursor tracking:** Separate position/time tracking from write logic

---

## 2. RenderClientBoundary Contract Concept

### Core Trait/Interface
```cpp
class IRenderClientBoundary {
public:
    virtual ~IRenderClientBoundary() = default;
    
    virtual std::expected<BufferAcquireResult, RenderClientFailure> 
        acquire_buffer(const BufferAcquireRequest& request) = 0;
    
    virtual std::expected<BufferReleaseResult, RenderClientFailure> 
        release_buffer(const BufferReleaseRequest& request) = 0;
    
    virtual std::expected<PaddingSnapshot, RenderClientFailure> 
        query_padding() const = 0;
    
    virtual std::expected<AvailableFramesSnapshot, RenderClientFailure> 
        query_available_frames() const = 0;
    
    virtual bool is_ready() const = 0;
    virtual bool has_error() const = 0;
    virtual std::optional<RenderClientFailure> last_error() const = 0;
};
```

### Design Principles
- **Platform-agnostic:** No Windows types in interface
- **Pure data types:** All inputs/outputs are value types
- **No raw pointers:** Buffer addresses not exposed at contract level
- **Error model:** All failures via `RenderClientFailure` enum
- **State queries:** `is_ready()`, `has_error()`, `last_error()` for state inspection

---

## 3. WriteRequest / WriteResult / WriteError Semantics

### WriteRequest Variants
| Variant | Purpose | Fields |
|---------|---------|--------|
| `Write` | Write audio frames | `frame_count`, `channel_count`, `sample_rate` |
| `Flush` | Flush buffered data | (none) |
| `Reset` | Reset to initial state | (none) |
| `Close` | Close writer | (none) |

### WriteResult Variants
| Variant | Meaning | Fields |
|---------|---------|--------|
| `Written` | Success, frames written | `frames_written`, `bytes_written` |
| `WouldBlock` | Buffer full, try later | `available_frames`, `requested_frames` |
| `Flushed` | Flush completed | (none) |
| `Closed` | Close completed | (none) |

### WriteError Variants
| Variant | Meaning | Context |
|---------|---------|---------|
| `DeviceClosed` | Writer already closed | (none) |
| `WouldBlock` | Buffer full | `available`, `requested` |
| `WriteFailed` | Write operation failed | `description` |
| `InvalidRequest` | Invalid parameters | `reason` |
| `BufferTooSmall` | Buffer insufficient | `available_frames`, `requested_frames` |
| `Internal` | Internal error | `description` |
| `FlushFailed` | Flush failed | `description` |

### Design Principles
- **Enum-based results:** No exceptions, no error codes
- **Rich context:** Each error carries relevant diagnostic data
- **Exhaustive matching:** Compiler enforces handling all cases
- **No side effects:** Pure data types, no hidden state

---

## 4. WriterState / WriterCursor Semantics

### WriterState Snapshot
```cpp
struct WriterState {
    // Buffer status
    size_t buffer_fill_frames;
    size_t buffer_capacity_frames;
    BufferLifecycle lifecycle;  // Empty, Partial, Full, Closed
    
    // Write tracking
    size_t write_head;
    size_t wrap_count;
    size_t total_frames_written;
    size_t total_bytes_written;
    
    // Error tracking
    size_t consecutive_would_blocks;
    size_t max_consecutive_would_blocks;
    
    // Health assessment
    bool is_healthy() const;
    BufferPressure pressure_level() const;
    std::vector<std::string> health_warnings() const;
};
```

### WriterCursor Position
```cpp
struct WriterCursor {
    size_t write_position_frames;
    size_t buffered_frames;
    size_t capacity_frames;
    size_t total_written_frames;
    
    // Time conversions
    double write_position_seconds(double sample_rate) const;
    double buffered_seconds(double sample_rate) const;
    double capacity_seconds(double sample_rate) const;
    double total_written_seconds(double sample_rate) const;
    
    // Millisecond precision
    int64_t write_position_millis(double sample_rate) const;
    int64_t buffered_millis(double sample_rate) const;
    int64_t capacity_millis(double sample_rate) const;
    int64_t total_written_millis(double sample_rate) const;
};
```

### Design Principles
- **Immutable snapshots:** State copied, not referenced
- **Derived metrics:** Health/pressure computed from raw data
- **Time abstraction:** Position in frames, convertible to time
- **Comparison support:** `changed_field_names()`, `summary_line()` for debugging

---

## 5. RenderClientFailure Mapping Table

| Failure Variant | Meaning | Fatal? | Recoverable? |
|-----------------|---------|--------|--------------|
| `DeviceLost` | Audio device disconnected | Yes | No |
| `BufferTooSmall` | Buffer insufficient | No | Yes (retry) |
| `PaddingUnavailable` | Padding query failed | No | Yes (retry) |
| `RenderClientUnavailable` | Client not ready | No | Yes (reconnect) |
| `InvalidRequest` | Invalid parameters | No | Yes (fix params) |
| `BufferAcquisitionFailed` | GetBuffer failed | No | Yes (retry) |
| `BufferReleaseFailed` | ReleaseBuffer failed | No | Yes (retry) |
| `Internal` | Internal error | No | Maybe |

### Mapping to WriteError
```cpp
WriteError map_render_client_failure(const RenderClientFailure& failure) {
    return std::visit(overloaded{
        [&](const DeviceLost&) { return WriteError::WriteFailed{"device lost"}; },
        [&](const BufferTooSmall& e) { 
            return WriteError::BufferTooSmall{e.available_frames, e.requested_frames}; 
        },
        [&](const PaddingUnavailable&) { return WriteError::Internal{"padding unavailable"}; },
        [&](const RenderClientUnavailable&) { return WriteError::DeviceClosed{}; },
        [&](const InvalidRequest& e) { return WriteError::InvalidRequest{e.reason}; },
        [&](const BufferAcquisitionFailed& e) { return WriteError::WriteFailed{e.description}; },
        [&](const BufferReleaseFailed& e) { return WriteError::WriteFailed{e.description}; },
        [&](const Internal& e) { return WriteError::Internal{e.description}; }
    }, failure);
}
```

---

## 6. Fake Backend Testing Philosophy

### Principles
1. **Fake = Real interface, simulated behavior**
2. **Inject failures:** Fake allows injecting errors at any point
3. **Track state:** Fake records all calls for verification
4. **No side effects:** Fake doesn't touch real hardware
5. **Deterministic:** Same inputs always produce same outputs

### FakeDeviceBufferWriter Pattern
```cpp
class FakeDeviceBufferWriter : public IDeviceBufferWriter {
public:
    // Configuration
    void set_failure_mode(std::optional<WriteError> error);
    void set_would_block_after(size_t frames);
    
    // State inspection
    size_t write_attempts() const;
    size_t writes_completed() const;
    size_t would_block_count() const;
    std::vector<WriteRequest> received_requests() const;
    
    // IDeviceBufferWriter implementation (simulated)
    WriteResult write(const WriteRequest& request) override;
    // ...
};
```

### FakeRenderClientBoundary Pattern
```cpp
class FakeRenderClientBoundary : public IRenderClientBoundary {
public:
    // Configuration
    void set_failure_mode(std::optional<RenderClientFailure> error);
    void set_ready(bool ready);
    void set_padding(size_t padding);
    
    // State inspection
    size_t acquire_count() const;
    size_t release_count() const;
    std::vector<BufferAcquireRequest> acquire_requests() const;
    
    // IRenderClientBoundary implementation (simulated)
    std::expected<BufferAcquireResult, RenderClientFailure> 
        acquire_buffer(const BufferAcquireRequest& request) override;
    // ...
};
```

---

## 7. Boundary Seam Testing Philosophy

### Cross-Boundary Contract Tests
- **Purpose:** Verify two implementations satisfy same behavioral contract
- **Pattern:** Test both `FakeRenderClientBoundary` and `WasapiDeviceBufferWriter` with same scenarios
- **Assertions:** Same inputs → same outputs/errors regardless of implementation

### Test Categories
1. **Happy path:** Normal operation succeeds
2. **Error injection:** Failures propagate correctly
3. **State transitions:** Lifecycle changes are correct
4. **Edge cases:** Boundary conditions handled
5. **Idempotency:** Repeated calls safe
6. **Order independence:** Operations don't depend on call order

---

## 8. Adapter Isolation Philosophy

### Core Principle
**Contract types must never expose platform types.**

### Layer Separation
```
┌─────────────────────────────────────┐
│  DeviceBufferWriter (pure contract) │
├─────────────────────────────────────┤
│  RenderClientBoundary (pure contract)│
├─────────────────────────────────────┤
│  Adapter Isolation Shell            │
│  (platform-specific, but no real API)│
├─────────────────────────────────────┤
│  Real Platform Implementation       │
│  (Windows WASAPI, macOS CoreAudio)  │
└─────────────────────────────────────┘
```

### Isolation Rules
1. **No platform imports in contract layer**
2. **No platform imports in adapter shell**
3. **Platform imports only in real implementation**
4. **All platform types hidden behind opaque pointers**
5. **All errors converted to contract error types**

---

## 9. Genealogy Structure Rules

### Family Tree Principle
- Files grouped by semantic family, not by type
- Related files in same directory
- Unrelated files in separate directories
- No flat file dumping grounds

### Example
```
render_client_boundary/
├── mod.rs                    # Module entry
├── trait_def.rs              # Interface definition
├── types/                    # Data types family
│   ├── mod.rs
│   ├── buffer_request.rs
│   ├── buffer_result.rs
│   ├── failure.rs
│   └── snapshot.rs
└── fake_client/             # Test implementation family
    ├── mod.rs
    ├── client.rs
    ├── buffer_simulation.rs
    ├── trait_impl.rs
    └── tests.rs
```

---

## 10. Single File Single Responsibility Rule

### Guidelines
- **One struct/class per file** (with related impl blocks)
- **One enum per file** (with related impl blocks)
- **One trait/interface per file** (with documentation)
- **Maximum 200 lines per production file** (target)
- **Maximum 250 lines per test file** (target)
- **>300 lines requires justification**
- **>400 lines is hard stop**

### Benefits
- Easy to find code
- Easy to understand code
- Easy to review code
- Easy to refactor code
- Easy to test code

---

## 11. Pre-Push Validation Rules

### Required Gates
1. **Format check:** `cargo fmt --check` / `clang-format --dry-run`
2. **Compilation check:** `cargo check` / `cmake --build`
3. **Lint check:** `cargo clippy` / `clang-tidy`
4. **Unit tests:** All tests pass
5. **Working tree clean:** No uncommitted changes
6. **No temporary files:** No `.tmp`, `.log`, `.bak` files

### Verification Script
```powershell
# verify_audio_backend.ps1
# - State Gate: branch, HEAD, working tree
# - Build Gate: compilation, linting
# - Test Gate: unit tests, integration tests
# - Diff Gate: no conflict markers
# - Fan-out Gate: file structure compliance
# - Temp Gate: no temporary artifacts
```

---

## 12. Anti-Pollution Principles

### Contract Layer Purity
- **No platform #include** in contract headers
- **No platform #ifdef** in contract code
- **No platform types** in contract interfaces
- **No platform callbacks** in contract callbacks
- **No platform errors** in contract error types

### Adapter Layer Isolation
- **No real API calls** in adapter shell
- **No COM pointers** in adapter shell
- **No Windows resources** in adapter shell
- **All operations return errors** in adapter shell
- **All state is pure metadata** in adapter shell

### Real Implementation Separation
- **Platform code isolated** in dedicated directory
- **Platform errors converted** to contract errors
- **Platform resources managed** with RAII
- **Platform cleanup guaranteed** with destructors
- **Platform failures handled** gracefully

---

## Recommended Next Project

### Project Name
**KivoAudioCoreCpp**

### First Task
**KIVO-CPP-AUDIO-CORE-BOOTSTRAP-001**

### Goal
Establish C++ audio core repository skeleton with:
1. Contract layer (IDeviceBufferWriter, IRenderClientBoundary)
2. WASAPI adapter family (real implementation)
3. FFmpeg decode seam (decode interface)
4. Fake backend testing system
5. CI validation rules

### Directory Structure
```
KivoAudioCoreCpp/
├── include/
│   ├── kivo/audio/
│   │   ├── device_buffer_writer.hpp    # IDeviceBufferWriter
│   │   ├── render_client_boundary.hpp  # IRenderClientBoundary
│   │   └── types/                      # WriteRequest, WriteResult, etc.
│   └── kivo/audio/wasapi/
│       └── render_client_adapter.hpp   # Real WASAPI adapter
├── src/
│   ├── core/
│   │   ├── device_buffer_writer.cpp
│   │   └── render_client_boundary.cpp
│   ├── wasapi/
│   │   ├── render_client_adapter.cpp
│   │   └── com_wrapper.cpp
│   └── fake/
│       ├── fake_device_buffer_writer.cpp
│       └── fake_render_client_boundary.cpp
├── tests/
│   ├── contract/
│   ├── boundary/
│   ├── adapter/
│   └── integration/
├── CMakeLists.txt
└── .github/
    └── workflows/
        └── ci.yml
```

---

## Summary

The Rust audio backend provides a complete architectural specification for the C++ audio core. All contracts are defined, all boundaries are tested, all fake backends are implemented, and all isolation patterns are established. The C++ project should inherit these patterns and extend them with real WASAPI implementation.

**Key Takeaway:** Contract purity is paramount. Never let platform types pollute the contract layer.
