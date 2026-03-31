use std::fs;
use std::path::Path;

fn ensure_dir_with_placeholder(dir: &Path, placeholder_name: &str, placeholder_content: &str) {
    let _ = fs::create_dir_all(dir);

    let has_any_file = match fs::read_dir(dir) {
        Ok(rd) => rd.flatten().any(|ent| ent.path().is_file()),
        Err(_) => false,
    };

    if !has_any_file {
        let placeholder_path = dir.join(placeholder_name);
        if !placeholder_path.exists() {
            let _ = fs::write(&placeholder_path, placeholder_content);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=TAURI_CONFIG");
    println!("cargo:rustc-check-cfg=cfg(desktop)");
    println!("cargo:rustc-cfg=desktop");
    println!("cargo:rustc-check-cfg=cfg(mobile)");

    // Keep these in sync with src-tauri/tauri.conf.json bundle.resources entries.
    let ffmpeg_root = Path::new("../third_party/ffmpeg");
    ensure_dir_with_placeholder(
            &ffmpeg_root.join("bin"),
            "PLACE_FFMPEG_DLLS_HERE.txt",
            "Place FFmpeg DLLs here (e.g. avcodec-*.dll, avformat-*.dll, avutil-*.dll, swresample-*.dll)
",
        );
    ensure_dir_with_placeholder(
        &ffmpeg_root.join("licenses"),
        "PLACE_LICENSE_TEXTS_HERE.txt",
        "Place FFmpeg license texts here (e.g. LICENSE.txt, COPYING.txt, etc.)
",
    );

    tauri_build::build();
}
