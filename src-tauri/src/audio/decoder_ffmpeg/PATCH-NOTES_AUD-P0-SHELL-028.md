# AUD-P0-SHELL-028 — swresample: 任意输入格式 -> f32 packed

## 范围与约束
- 仅修改/新增：`src-tauri/src/audio/decoder_ffmpeg/**`
- `src/**`、`public/**`：0 改动
- 未新增第三方依赖

## 改动清单（逐文件）

### 新增
- `src-tauri/src/audio/decoder_ffmpeg/resample.rs`
  - 新增 `FfmpegResampler`（SwrContext wrapper），将任意输入 `sample_fmt`（含 planar/packed, s16/s32 等）统一转换为 **f32 packed(interleaved)**。
  - `swr_init/swr_convert` 等失败时，错误信息带 `in_fmt/out_fmt/ch/sr/layout` 诊断串。

### 修改
- `src-tauri/src/audio/decoder_ffmpeg/ffi.rs`
  - 补齐 swresample 相关类型/函数指针：`SwrContext`、`swr_alloc_set_opts/swr_init/swr_free/swr_get_out_samples/swr_convert`
  - 增加 `AV_SAMPLE_FMT_FLT`，以及 `av_get_default_channel_layout/av_get_sample_fmt_name`
- `src-tauri/src/audio/decoder_ffmpeg/api.rs`
  - `FfmpegApi` 增加并加载 swresample/辅助符号（对外 API 签名不变）
- `src-tauri/src/audio/decoder_ffmpeg/decoder/mod.rs`
  - `FfmpegDecoder` 内部新增 `sample_rate_hint` + `resampler`（lazy 初始化；seek 后重置）
- `src-tauri/src/audio/decoder_ffmpeg/decoder/open.rs`
  - 打开文件时设置 `sample_rate_hint`（优先 header probe，否则 44_100）
- `src-tauri/src/audio/decoder_ffmpeg/decoder/read.rs`
  - 解码帧输出改为走 `resample::frame_to_pcm_f32`（不再假定输入本来就是 f32）
- `src-tauri/src/audio/decoder_ffmpeg/header_probe.rs`
  - 增加 WAV 头探测 sample rate
- `src-tauri/src/audio/decoder_ffmpeg/mod.rs`
  - 注册 `resample` 子模块（内部下沉；对外 `pub use` 不破坏）

> 备注：旧 `convert.rs` 仍保留在目录，但不再被 `mod.rs` 引用（不会参与编译）。

## 手工验证（Windows）
1) 设置 FFmpeg DLL 目录（二选一）：
   - `$env:KIVO_FFMPEG_DIR = "C:\\ffmpeg\\bin"`
   - 或 `ffmpeg_smoke` 参数：`--ffmpeg-dir "C:\\ffmpeg\\bin"`
2) 运行 smoke（mp3/flac/wav 各跑 1 次）：
   - `cargo run --bin ffmpeg_smoke -- "C:\\music\\sample.mp3" "C:\\temp\\out_mp3.wav" 0 10`
   - `cargo run --bin ffmpeg_smoke -- "C:\\music\\sample.flac" "C:\\temp\\out_flac.wav" 0 10`
   - `cargo run --bin ffmpeg_smoke -- "C:\\music\\sample.wav" "C:\\temp\\out_wav.wav" 0 10`
3) 播放输出 wav，确认无明显崩音/全错音。

## 必跑 Gates（Windows）
```powershell
cd src-tauri
cargo clean
cargo fmt
cargo test
cargo clippy -- -D warnings
```
