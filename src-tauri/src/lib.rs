use audiotags::Tag;
use serde_json::{json, Value};
use std::fs::{self, DirEntry};
use std::path::PathBuf;

// 定义Song结构体
#[derive(Debug)]
struct Song {
    name: String,
    id: u32,
    ar: Vec<Artist>,
    lyric: String,
    al: Album,
    src: PathBuf,
}

// 定义艺术家结构体
#[derive(Debug)]
struct Artist {
    id: u32,
    name: String,
    alias: Vec<String>,
}

// 定义专辑结构体
#[derive(Debug)]
struct Album {
    id: u32,
    name: String,
    pic_url: String,
}

// 检查文件是否是音乐文件的辅助函数
fn is_music_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    match path.extension() {
        Some(ext) => {
            ext.to_string_lossy().eq_ignore_ascii_case("mp3")
                || ext.to_string_lossy().eq_ignore_ascii_case("ogg")
                || ext.to_string_lossy().eq_ignore_ascii_case("flac")
        }
        None => false,
    }
}

// 递归扫描文件夹的辅助函数
fn scan_music_files(dir: PathBuf) -> Vec<PathBuf> {
    let mut music_files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let nested_files = scan_music_files(entry.path());
                music_files.extend(nested_files);
            } else if is_music_file(&entry) {
                music_files.push(entry.path());
            }
        }
    }
    music_files
}

// 解析音乐文件并返回Song结构体的函数
fn parse_music_file(file: PathBuf) -> Result<Song, String> {
    let tag = Tag::new().read_from_path(&file).unwrap();
    // let tag = AnyTag::new(&file);
    let title = tag.title().unwrap_or("Unknown Title").to_string();
    let artist = tag.artist().unwrap_or("Unknown Artist").to_string();
    let album_title = tag.album_title().unwrap_or("Unknown Album").to_string();
    // 这里简化处理，只取第一个艺术家和专辑，并且没有处理歌词和图片
    let song = Song {
        name: title,
        id: 0, // 示例中未提供ID生成逻辑
        ar: vec![Artist {
            id: 0,
            name: artist,
            alias: vec![],
        }],
        lyric: String::new(), // 示例中未提供歌词获取逻辑
        al: Album {
            id: 0,
            name: album_title,
            pic_url: String::new(), // 示例中未提供图片获取逻辑
        },
        src: file,
    };
    Ok(song)
}

// 将Song结构体转换为JSON字符串的方法
impl Song {
    fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "id": self.id,
            "ar": self.ar.iter().map(|ar| {
                json!({
                    "id": ar.id,
                    "name": ar.name,
                    "alias": ar.alias,
                })
            }).collect::<Vec<Value>>(),
            "lyric": self.lyric,
            "al": {
                "id": self.al.id,
                "name": self.al.name,
                "picUrl": self.al.pic_url,
            },
            "src": self.src.display().to_string(),
        })
    }
}

// Tauri命令获取音乐列表
#[tauri::command]
fn get_music_list() -> Result<Vec<Value>, String> {
    let music_dir = dirs::audio_dir().ok_or("无法找到音乐文件夹")?;
    let music_files = scan_music_files(music_dir);
    let mut songs_json = Vec::new();
    for file in music_files {
        match parse_music_file(file) {
            Ok(song) => songs_json.push(song.to_json()),
            Err(e) => return Err(e),
        }
    }
    Ok(songs_json)
}

// Tauri应用入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_music_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
