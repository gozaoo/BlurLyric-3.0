use std::fs::{self, DirEntry};
use std::path::PathBuf;
use tauri::Manager;

// 定义一个辅助函数来检查文件是否是音乐文件
fn is_music_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    match path.extension() {
        Some(ext) => ext.to_string_lossy().eq_ignore_ascii_case("mp3")
            || ext.to_string_lossy().eq_ignore_ascii_case("ogg")
            || ext.to_string_lossy().eq_ignore_ascii_case("flac"),
        None => false,
    }
}

// 定义一个辅助函数来递归扫描文件夹
fn scan_music_files(dir: PathBuf) -> Vec<PathBuf> {
    let mut music_files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                // 如果是文件夹，递归调用
                let nested_files = scan_music_files(entry.path());
                music_files.extend(nested_files);
            } else if is_music_file(&entry) {
                // 如果是音乐文件，添加到列表中
                music_files.push(entry.path());
            }
        }
    }
    music_files
}

#[tauri::command]
fn getMusicList() -> Result<Vec<String>, String> {
    // 获取用户音乐文件夹的路径
    // 注意：这里的路径是示例，您需要根据实际操作系统进行调整
    let music_dir = dirs::audio_dir().ok_or("无法找到音乐文件夹")?;

    // 扫描音乐文件夹并收集音乐文件
    let music_files = scan_music_files(music_dir);

    // 将文件路径转换为字符串并返回
    Ok(music_files.into_iter().map(|path| path.to_string_lossy().into_owned()).collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![getMusicList])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
