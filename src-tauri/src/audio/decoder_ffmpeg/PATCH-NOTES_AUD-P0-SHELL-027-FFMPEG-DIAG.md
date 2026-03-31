# PATCH-NOTES — AUD-P0-SHELL-027-FFMPEG-DIAG

范围：仅修改 `src-tauri/**`（不触碰 `src/**`、`public/**`），且不新增第三方依赖。

## 目标对齐
- 动态库加载/符号解析报错包含：library、symbol、加载路径策略(dir/env/os-default)、WinAPI os_error（如有）
- `ffmpeg_smoke` 失败 summary 里包含“原因 + 定位线索（hint）”
- 外层 API 不变：`FfmpegLoadOptions`、`FfmpegApi::load` 等签名保持不变
- 可维护性：小函数/模块化；新增字段不引入破坏性外部变更

## 文件级变更
### 修改
- `src-tauri/src/audio/errors.rs`
  - 修复 `thiserror` format string（原来包含非法占位符 `tri...`，会导致编译期格式化错误）。
  - `FfmpegLibraryNotFound` 增加 `search` 字段，并在 Display 中输出：`search=... tried=... last_error=...`
  - `FfmpegSymbolMissing` 增加 `search` 与 `os_error` 字段，Display 输出包含 symbol/library/search/os_error

- `src-tauri/src/audio/decoder_ffmpeg/dll_set.rs`
  - `resolve_ffmpeg_dir()` 返回 `(dir, search)`：明确区分来源：
    - `source=arg(ffmpeg_dir)`
    - `source=env(KIVO_FFMPEG_DIR)`
    - `source=os_default_search`
  - `load_one()` 传递 `search` 给 `DynLib::load`，并在 `FfmpegLibraryNotFound` 中携带 `search`

- `src-tauri/src/audio/decoder_ffmpeg/dynlib.rs`
  - `DynLib` 增加 `search` 字段（用于在 symbol missing 时输出加载策略）
  - `DynLib::load(name, dir, search)` 内部 API 变更（对外仍是 decoder_ffmpeg 私有模块，不影响外层）
  - 符号缺失时，返回 `AudioError::FfmpegSymbolMissing { symbol, library, search, os_error }`

- `src-tauri/src/audio/decoder_ffmpeg/dynlib_windows.rs`
  - `load_library(name, dir, search)`：错误 `LoadLibraryW failed` 增加 `search=...`
  - 仍使用 RAII 临时 `SetDllDirectoryW(dir)` 并在 drop 恢复，避免污染全局 DLL 搜索路径

- `src-tauri/src/bin/ffmpeg_smoke.rs`
  - 在 summary 中新增：
    - `cause=missing_dll|missing_symbol|decode_failed|unknown`
    - `hint=...`
  - 不改变既有交互风格（仍打印原有 summary + error 字段）

## 如何验证
1) Gates：
```bat
cd src-tauri
cargo clean
cargo fmt
cargo test
cargo clippy -- -D warnings
```

2) 手工验证（任选其一）：
- 缺 DLL（不设置 KIVO_FFMPEG_DIR / 或指向空目录）：
```bat
cd src-tauri
cargo run --bin ffmpeg_smoke -- "C:\\music\\test.mp3"
```
期望：error 中包含 `library=... search=source=... os_error=...`，summary 中 `cause=missing_dll` + hint。

- DLL 存在（设置正确目录）：
```bat
set KIVO_FFMPEG_DIR=C:\\ffmpeg\\bin
cargo run --bin ffmpeg_smoke -- "C:\\music\\test.mp3"
```
期望：能输出可读的成功信息；若失败，summary cause/hint 仍齐全。


### 新增
- `src-tauri/src/bin/ffmpeg_smoke_support.rs`
  - 将 `ffmpeg_smoke` 的参数解析/usage/错误分类/summary 辅助函数下沉到独立模块，保持 `ffmpeg_smoke.rs` 主逻辑精简（单文件 <200 行）。
