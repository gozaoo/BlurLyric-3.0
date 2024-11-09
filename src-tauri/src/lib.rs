use audiotags::Tag;
use serde_json::{json, Value};
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

lazy_static::lazy_static! {
    static ref SONG_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref ARTIST_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref ALBUM_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref MUSIC_CACHE: Mutex<HashMap<PathBuf, Vec<Song>>> = Mutex::new(HashMap::new());

}


fn cache_music_list(dir: PathBuf, songs: Vec<Song>) {
    let mut cache = MUSIC_CACHE.lock().unwrap();
    cache.insert(dir, songs);
}
fn next_song_id() -> u32 {
    let mut id = SONG_ID_COUNTER.lock().unwrap();
    *id += 1;
    *id
}

fn next_artist_id() -> u32 {
    let mut id = ARTIST_ID_COUNTER.lock().unwrap();
    *id += 1;
    *id
}

fn next_album_id() -> u32 {
    let mut id = ALBUM_ID_COUNTER.lock().unwrap();
    *id += 1;
    *id
}

// 定义Song结构体
#[derive(Debug,Serialize, Deserialize)]
struct Song {
    name: String,
    id: u32,
    ar: Vec<Artist>,
    lyric: String,
    al: Album,
    src: PathBuf,
}

// 定义艺术家结构体

#[derive(Debug,Serialize, Deserialize)]
struct Artist {
    id: u32,
    name: String,
    alias: Vec<String>,
}


#[derive(Debug,Serialize, Deserialize)]
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
    save_cache_to_disk();
    music_files
}

// 独立的方法，用于添加用户音乐文件夹
fn add_users_music_dir() {
    if let Some(audio_dir) = dirs::audio_dir() {
        let mut music_dirs = MUSIC_DIRS.lock().unwrap();
        music_dirs.push(audio_dir);
        save_music_dirs_to_disk();
    }
}

// 程序启动时调用的方法
fn init_application() {
    load_cache_from_disk();
    load_music_dirs_from_disk();

    // 检测 MUSIC_DIRS 是否为空，如果为空，则添加用户音乐文件夹
    {
        let music_dirs = MUSIC_DIRS.lock().unwrap();
        if music_dirs.is_empty() {
            add_users_music_dir();
        }
    }

    // 重新加载音乐目录，以包含用户音乐文件夹
    load_music_dirs_from_disk();
}

// 解析音乐文件并返回Song结构体的函数
fn parse_music_file(file: PathBuf) -> Result<Song, String> {
    let tag = Tag::new().read_from_path(&file).unwrap();
    // let tag = AnyTag::new(&file);
    let title = tag.title().unwrap_or("Unknown Title").to_string();
    let artist = tag.artist().unwrap_or("Unknown Artist").to_string();
    let album = tag.album().unwrap();
    // 这里简化处理，只取第一个艺术家和专辑，并且没有处理歌词和图片
    let song = Song {
        name: title,
        id: next_song_id(), // 示例中未提供ID生成逻辑
        ar: vec![Artist {
            id: next_artist_id(),
            name: artist,
            alias: vec![],
        }],
        lyric: String::new(), // 示例中未提供歌词获取逻辑
        al: Album {
            id: next_album_id(),
            name: album.title.to_string(),
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
    let cache = MUSIC_CACHE.lock().unwrap();
    let mut songs_json = Vec::new();
    for songs in cache.values() {
        for song in songs {
            songs_json.push(song.to_json());
        }
    }
    Ok(songs_json)
}
#[tauri::command]
fn get_album_cover(album_id: u32) -> Result<Vec<u8>, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    for songs in cache.values() {
        for song in songs {
            if song.al.id == album_id {
                // 使用 match 语句来处理 Option
                match Tag::new().read_from_path(&song.src).unwrap().album_cover() {
                    Some(cover) => {
                        // 现在我们可以安全地访问 cover 的 data 字段
                        return Ok(cover.data.to_vec());
                    },
                    None => {
                        // 如果没有封面，继续循环
                        continue;
                    }
                }
            }
        }
    }
    // 如果循环结束还没有找到封面，返回错误
    Err("Album cover not found".into())
}


fn save_cache_to_disk() {
    let cache = MUSIC_CACHE.lock().unwrap();
    let file = std::fs::File::create("music_cache.json").unwrap();
    serde_json::to_writer(file, &*cache).unwrap();
}

fn load_cache_from_disk() {
    if let Ok(file) = std::fs::File::open("music_cache.json") {
        let cache: HashMap<PathBuf, Vec<Song>> = serde_json::from_reader(file).unwrap();
        let mut cached = MUSIC_CACHE.lock().unwrap();
        *cached = cache;
    }
}
#[tauri::command]
fn get_music_file(song_id: u32) -> Result<Vec<u8>, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    for songs in cache.values() {
        for song in songs {
            if song.id == song_id {
                return Ok(std::fs::read(&song.src).map_err(|e| e.to_string())?);
            }
        }
    }
    Err("Music file not found".into())
}
#[tauri::command]
fn get_all_music_dirs() -> Result<Vec<String>, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    let dirs = cache.keys().map(|path| path.display().to_string()).collect();
    Ok(dirs)
}
#[tauri::command]
fn remove_music_dirs(dirs_to_remove: Vec<String>) -> Result<(), String> {
    let mut music_dirs = MUSIC_DIRS.lock().unwrap();
    music_dirs.retain(|dir| !dirs_to_remove.contains(&dir.display().to_string()));

    let mut cache = MUSIC_CACHE.lock().unwrap();
    for dir_str in dirs_to_remove {
        let dir = PathBuf::from(dir_str);
        cache.remove(&dir);
    }
    
    save_music_dirs_to_disk();
    save_cache_to_disk();
    Ok(())
}

#[tauri::command]
fn add_music_dirs(new_dirs: Vec<String>) -> Result<(), String> {
    let mut music_dirs = MUSIC_DIRS.lock().unwrap();
    music_dirs.extend(new_dirs.iter().map(PathBuf::from));

    for dir_str in new_dirs {
        let dir = PathBuf::from(dir_str);
        if dir.is_dir() {
            let songs = scan_music_files(dir.clone());
            let parsed_songs: Result<Vec<Song>, String> = songs.into_iter().map(parse_music_file).collect();
            if let Ok(parsed_songs) = parsed_songs {
                cache_music_list(dir, parsed_songs);
            }
        } else {
            return Err("One or more paths are not valid directories".into());
        }
    }
    save_music_dirs_to_disk();
    save_cache_to_disk();
    Ok(())
}
lazy_static::lazy_static! {
    static ref MUSIC_DIRS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
}

fn save_music_dirs_to_disk() {
    let dirs = MUSIC_DIRS.lock().unwrap();
    let file = std::fs::File::create("music_dirs.json").unwrap();
    serde_json::to_writer(file, &*dirs).unwrap();
}

fn load_music_dirs_from_disk() {
    if let Ok(file) = std::fs::File::open("music_dirs.json") {
        let dirs: Vec<PathBuf> = serde_json::from_reader(file).unwrap();
        let mut music_dirs = MUSIC_DIRS.lock().unwrap();
        *music_dirs = dirs;
    }
}

#[tauri::command]
fn refresh_music_cache() -> Result<(), String> {
    let music_dirs = MUSIC_DIRS.lock().unwrap();
    let mut new_cache = HashMap::new();

    for dir in &*music_dirs {
        if dir.is_dir() {
            let songs = scan_music_files(dir.clone());
            let parsed_songs: Result<Vec<Song>, String> = songs.into_iter().map(parse_music_file).collect();
            if let Ok(parsed_songs) = parsed_songs {
                new_cache.insert(dir.clone(), parsed_songs);
            } else {
                return Err("Failed to parse some music files".into());
            }
        } else {
            return Err("One or more registered paths are not valid directories".into());
        }
    }

    let mut cache = MUSIC_CACHE.lock().unwrap();
    *cache = new_cache;

    save_cache_to_disk();
    Ok(())
}


// Tauri应用入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_application();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_music_list,
            get_all_music_dirs,
            add_music_dirs,
            remove_music_dirs,
            get_album_cover,
            get_music_file,
            refresh_music_cache // 添加新的命令到这里
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
