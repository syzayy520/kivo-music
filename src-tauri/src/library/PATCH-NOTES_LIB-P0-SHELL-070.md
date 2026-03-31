# PATCH-NOTES — LIB-P0-SHELL-070

## 目标
- 新增 `library` 模块门面（稳定 API）：`run_scan / counts / search_prefix`
- 新增并注册 Tauri commands（命名固定）：`library_scan_run / library_counts / library_search_prefix`
- 错误统一返回 `String`（稳定前缀 + 人类可读），不裸 Debug

## 新增/修改文件清单（逐文件）
- **新增** `src-tauri/src/library/mod.rs`  
  - library 模块门面；re-export 稳定 API 与类型。
- **新增** `src-tauri/src/library/types.rs`  
  - `ScanSummary / LibraryCounts / TrackHit`（serde 可序列化，供 tauri boundary 使用）。
- **新增** `src-tauri/src/library/errors.rs`  
  - `LibraryError`（thiserror）+ `to_public_string(prefix)`（错误前缀规范）。
- **新增** `src-tauri/src/library/store.rs`  
  - 进程内最小 store（OnceLock + Mutex），用于 P0 scan/count/search 的闭环；后续可替换成 SQLite，不影响对外 API。
- **新增** `src-tauri/src/library/scan.rs`  
  - `scan_roots()`：递归扫描 roots（支持常见音频后缀），构造 TrackHit 列表与 ScanSummary。
- **新增** `src-tauri/src/library/search.rs`  
  - `search_prefix()`：title / filename 前缀匹配（case-insensitive），`limit` 上限 200。
- **新增** `src-tauri/src/library/facade.rs`  
  - 稳定 API 实现：`run_scan / counts / search_prefix`（scan 成功后 replace store）。
- **新增** `src-tauri/src/library/commands.rs`  
  - Tauri commands：`library_scan_run / library_counts / library_search_prefix`（错误统一 `String`）。
- **修改** `src-tauri/src/lib.rs`  
  - `pub mod library;`  
  - invoke_handler 注册 3 个 library commands。

## Command 签名（固定）
- `library_scan_run(roots: Vec<String>) -> Result<ScanSummary, String>`
- `library_counts() -> Result<LibraryCounts, String>`
- `library_search_prefix(q: String, limit: u32) -> Result<Vec<TrackHit>, String>`

## 返回结构（serde）
- `ScanSummary { roots, tracks_indexed, skipped_files, errors, elapsed_ms }`
- `LibraryCounts { roots, tracks }`
- `TrackHit { id, title, path }`

## 错误前缀规范
- scan 入口失败：`[LIB_SCAN_FAILED] ...`
- 扫描过程中的 IO 失败：写入 `ScanSummary.errors`（例如 `[SCAN_READ_DIR_FAILED] <path>: <reason>`）

## 验证（本机）
```powershell
cd src-tauri
cargo fmt
cargo test
cargo clippy -- -D warnings
```

### 最小手工验证（无需前端接线）
1) 运行一次扫描（通过 tauri invoke 或后续你加的前端按钮）：
   - roots 传入一个包含少量音频文件的目录
   - 期望：返回 `tracks_indexed > 0`（或 errors 里有清晰原因）
2) 调用 `library_counts()`：期望 tracks/roots 与 scan 结果一致
3) 调用 `library_search_prefix("a", 20)`：期望返回 title 或文件名前缀匹配的 TrackHit 列表
