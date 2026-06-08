# Kivo Backend Genealogy Checklist

## Kivo Backend Genealogy Law

Kivo Music backend uses **整树家谱模式** (Whole-Tree Genealogy Mode). This is not a suggestion — it is an iron law.

### Core Rules

| Rule | Description |
|------|-------------|
| Folder = Feature Family | Each folder represents one functional module family |
| Subfolder = Subfamily | Each subfolder represents a sub-function within the family |
| File = Smallest Responsibility | Each file has exactly one responsibility, one purpose |
| mod.rs = Registry Only | `mod.rs` files contain only re-exports, module declarations, and doc comments — no logic |

### Family Ownership Map Rule

Every file must belong to exactly one family directory. The directory tree structure must reflect the module's functional relationship to its parent.

```
playback/
├── backends/                    [Engine Layer]
│   ├── native/                  [Native engine family]
│   └── native_engine_tests/     [Engine tests]
├── native_pipeline/             [Pipeline Layer]
│   ├── seek/                    [Seek sub-family]
│   ├── seek_transaction/        [Transaction sub-family]
│   └── tests/                   [Pipeline tests]
├── output_wasapi/               [Output Layer]
│   ├── buffer/                  [Buffer sub-family]
│   ├── client/                  [Client sub-family]
│   ├── device/                  [Device sub-family]
│   └── ...
├── playback_event/              [Event Layer]
├── production_output_route/     [Route Layer]
├── wasapi_seek_barrier_contract/ [Contract Layer]
├── manager/                     [Manager Layer]
└── ...
```

### Dependency Direction Audit

| Rule | Description |
|------|-------------|
| Parent imports child | Parent modules may import their direct child modules |
| No reverse imports | Child modules must NOT import parent modules |
| No sibling cross-domain | Sibling modules in different domains must NOT import each other |
| No circular dependencies | Import graph must be a DAG (directed acyclic graph) |

**Verification**: `cargo check` catches circular dependencies. Manual audit required for cross-domain violations.

### Single Responsibility File Gate

Each file must have:

1. Exactly one `/// Purpose:` doc comment at the top
2. One primary responsibility (state management OR I/O OR error types OR test helpers — never multiple)
3. Max 220 lines (hard limit)

**Violations**:
- File mixes state management and I/O → Split
- File contains both production logic and test logic → Split
- File exceeds 220 lines → STOP, split before continuing

### mod.rs Rules

`mod.rs` files are restricted to:

```rust
// ALLOWED in mod.rs:
mod child_module;
pub use child_module::PublicType;
pub use child_module::public_function;

/// Module-level documentation
```

```rust
// FORBIDDEN in mod.rs:
fn any_function() { ... }        // No logic
struct AnyType { ... }           // No types (except re-exports)
impl AnyType { ... }             // No implementations
use crate::...;                  // No imports (except re-exports)
```

### Forbidden File Names

The following file names are forbidden unless explicitly approved by a dedicated audit ticket:

| Name | Reason |
|------|--------|
| `helper.rs` | Vague responsibility, becomes a dump file |
| `utils.rs` | Vague responsibility, becomes a dump file |
| `glue.rs` | Vague responsibility, becomes a dump file |
| `facade.rs` | Vague responsibility, becomes a dump file |
| `bridge.rs` | Vague responsibility, becomes a dump file |
| `common.rs` | Vague responsibility, becomes a dump file |
| `misc.rs` | Vague responsibility, becomes a dump file |

**Exception**: Files in `output_wasapi/frame_bridge/`, `output_wasapi/runtime_queue_bridge/`, `playback_event/bridge.rs`, `audio_bridge/` are approved legacy exceptions.

### Forbidden Module Inception

Module inception (folder name = file name inside) is forbidden:

```
// FORBIDDEN:
failure/failure.rs
state/state.rs
buffer/buffer.rs
client/client.rs

// CORRECT:
failure/mod.rs
state/mod.rs
buffer/mod.rs
client/mod.rs
```

### Playback Root Flat File Ban

The `playback/` root directory must not contain flat implementation files (only `mod.rs` and subdirectories). All implementation must live in family subdirectories.

### cfg(test) Seam Policy

- Production code must NOT contain `#[cfg(test)]` blocks
- Test code lives in separate test files or test directories
- Test helpers live in dedicated `helpers.rs` within test directories
- Test modules mirror production module structure

### Public API / Re-export Policy

- Each family directory's `mod.rs` declares its public API via `pub use`
- Internal types are `pub(crate)` or private
- Cross-family imports use the public API only
- No `pub(super)` for cross-family access

### Output/WASAPI vs Seek/Native Pipeline Boundary

| Module | Responsibility | Boundary |
|--------|---------------|----------|
| `output_wasapi/` | Output sink, ring buffer, WASAPI device, render thread | Owns all output-side state |
| `native_pipeline/` | Pipeline orchestration, seek coordination, decoder management | Owns pipeline-side state |
| `wasapi_seek_barrier_contract/` | Contract types for seek barrier synchronization | Shared contract boundary |

**Rule**: `output_wasapi` and `native_pipeline` must NOT directly import each other's internal types. Communication goes through the contract boundary only.

## 28-Item Checklist

### Tree Compliance (8 items)

1. Every new file belongs to exactly one family directory
2. Every family directory maps to one functional module
3. No file exists at the wrong tree depth
4. No orphan files (files without a parent directory matching their domain)
5. Test files mirror production file paths
6. `mod.rs` files only contain re-exports and module declarations (no logic)
7. No file name contains forbidden names (helper, utils, glue, facade, bridge, common, misc) unless explicitly approved
8. Directory names use snake_case and match their single responsibility

### Dependency Direction (4 items)

9. Parent modules import child modules (not reverse)
10. No sibling-to-sibling cross-domain imports
11. No circular dependencies
12. `use` statements reference only the file's own domain or declared public interfaces

### Single Responsibility (4 items)

13. Each file has exactly one `/// Purpose:` doc comment
14. No file contains both production logic and test logic
15. No file contains both state management and I/O operations
16. Error types are in dedicated error files, not inline

### Side Effect (4 items)

17. Filesystem operations are in dedicated I/O modules
18. Thread spawning is in dedicated lifecycle modules
19. Global state mutations are in dedicated state modules
20. External API calls are in dedicated boundary modules

### Evidence (4 items)

21. Every gate result is recorded in the Evidence Ledger
22. Every test run includes full output capture
23. Every line count is measured, not estimated
24. Every classification has a justification

### Genealogy (4 items)

25. Family ownership map is consistent with file placement
26. No module violates its declared layer
27. Cross-layer integration has a dedicated audit ticket reference
28. Public API surface matches the module's declared responsibility
