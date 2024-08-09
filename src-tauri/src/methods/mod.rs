// 导入子模块
pub mod libs;
pub use libs::folder;
pub use libs::config;

// 导入 Tauri 的相关模块
use tauri::{Manager};

// 注册 Tauri 事件
pub fn run_tauri_methods(app: &tauri::App) {
    // 注册 "scan-folder" 事件
    let main_window = app.get_window("main").expect("failed to get main window");
    main_window.listen_global("scan-folder", move |event| {
        if let Some(path) = event.payload::<String>() {
            if let Some(music_list) = folder::scan_folder(path) {
                // 这里可以处理扫描到的音乐列表，例如发送到前端
                println!("Scanned music list: {:?}", music_list);
            }
        }
    });
}

// 其他可能的方法和类型定义
