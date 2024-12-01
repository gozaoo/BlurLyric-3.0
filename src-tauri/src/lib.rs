use audiotags::Tag;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::{self, DirEntry};
// use base64::{encode};
use std::path::PathBuf;
use std::io;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

use tokio::fs as async_fs;
// use ;

// use audiotags::{Tag, Box<dyn AudioTag + Send + Sync>};
// use audiotags::Error as AudioTagError;
lazy_static! {
    static ref SONG_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref ARTIST_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref ALBUM_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref MUSIC_CACHE: Mutex<HashMap<PathBuf, Vec<Song>>> = Mutex::new(HashMap::new());
    static ref MUSIC_DIRS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
}
 
#[derive(Debug, Serialize, Deserialize)]
struct Song {
    name: String,
    id: u32,
    ar: Vec<Artist>,
    lyric: String,
    al: Album,
    src: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Artist {
    id: u32,
    name: String,
    alias: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Album {
    id: u32,
    name: String,
    pic_url: String,
}

// ID生成器
fn next_id(counter: &Mutex<u32>) -> u32 {
    let mut id = counter.lock().unwrap();
    *id += 1;
    *id
}

// 文件是否是音乐文件
fn is_music_file(entry: &DirEntry) -> bool {
    matches!(
        entry.path().extension().and_then(|ext| ext.to_str()),
        Some("mp3" | "ogg" | "flac")
    )
}

// 扫描文件夹中的音乐文件
fn scan_music_files(dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .ok()
        .into_iter()
        .flat_map(|entries| {
            entries.filter_map(Result::ok).flat_map(|entry| {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    scan_music_files(&entry.path())
                } else if is_music_file(&entry) {
                    vec![entry.path()]
                } else {
                    vec![]
                }
            })
        })
        .collect()
}


// 假设Song, Artist, Album, Tag, next_id和SONG_ID_COUNTER等都已经定义
fn parse_music_file(file: PathBuf) -> Result<Song, String> {
    // 尝试读取标签
    let tag_result = Tag::new().read_from_path(&file);

    // 获取文件名作为备用标题
    let file_name = file.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown Title")
        .to_string();

    // 根据标签读取结果处理
    let song_data = match tag_result {
        Ok(tag) => {
            // 如果成功读取标签，则使用标签中的信息
            let title = tag.title().map(|s| s.to_string()).unwrap_or_else(|| file_name.clone());
            let artist = tag.artist().map(|s| s.to_string()).unwrap_or_else(|| "Unknown Artist".to_string());
            let album = tag.album_title().map(|s| s.to_string()).unwrap_or_else(|| "Unknown Album".to_string());
            (title, artist, album)
        },
        Err(_) => {
            // 如果读取标签失败，则使用默认值
            (file_name.clone(), "Unknown Artist".to_string(), "Unknown Album".to_string())
        },
    };

    Ok(Song {
        name: song_data.0,
        id: next_id(&SONG_ID_COUNTER),
        ar: vec![Artist {
            id: next_id(&ARTIST_ID_COUNTER),
            name: song_data.1,
            alias: vec![],
        }],
        lyric: String::new(),
        al: Album {
            id: next_id(&ALBUM_ID_COUNTER),
            name: song_data.2,
            pic_url: String::new(),
        },
        src: file,
    })
}

// 缓存音乐列表
fn cache_music_list(dir: PathBuf, songs: Vec<Song>) {
    MUSIC_CACHE.lock().unwrap().insert(dir, songs);
}

// Tauri命令
#[tauri::command]
fn get_music_list() -> Result<Vec<Value>, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    Ok(cache.values().flat_map(|songs| songs.iter().map(|song| song.to_json())).collect())
}

#[tauri::command]
fn refresh_music_cache() -> Result<(), String> {
    let music_dirs = MUSIC_DIRS.lock().unwrap();
    let mut new_cache = HashMap::new();

    for dir in &*music_dirs {
        if dir.is_dir() {
            let songs = scan_music_files(dir);
            let parsed_songs: Result<Vec<Song>, String> = songs.into_iter().map(parse_music_file).collect();
            match parsed_songs {
                Ok(songs) => {
                    new_cache.insert(dir.clone(), songs);
                }
                Err(e) => {
                    eprintln!("Error parsing music files in {}: {}", dir.display(), e);
                    return Err(e);
                }
            }
        } else {
            return Err(format!("Path is not a directory: {}", dir.display()));
        }
    }

    *MUSIC_CACHE.lock().unwrap() = new_cache;
    Ok(())
}

// Song结构体JSON序列化
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




// 独立的方法，用于添加用户音乐文件夹
fn add_users_music_dir() {
    if let Some(audio_dir) = dirs::audio_dir() {
        let audio_dir_path = audio_dir.to_str().unwrap().to_string();
        let _ = add_music_dirs(vec![audio_dir_path]);
    }
}

// 程序启动时调用的方法
fn init_application() {
    // 加载音乐缓存
    if let Err(e) = load_cache_from_disk() {
        eprintln!("Failed to load music cache from disk: {}", e);
    }

    // 加载音乐目录
    if let Err(e) = load_music_dirs_from_disk() {
        eprintln!("Failed to load music directories from disk: {}", e);
    }
    eprintln!("test");

    // 检查音乐目录是否为空
    {
        let mut music_dirs = MUSIC_DIRS.lock().unwrap();
        if music_dirs.is_empty() {
            if let Some(audio_dir) = dirs::audio_dir() {
                music_dirs.push(audio_dir);
                if let Err(e) = save_music_dirs_to_disk() {
                    eprintln!("Failed to save updated music directories to disk: {}", e);
                }
            } else {
                eprintln!("No default audio directory found.");
            }
        }
    }

    // 刷新音乐缓存
    if let Err(e) = refresh_music_cache() {
        eprintln!("Failed to refresh music cache: {}", e);
    }
}


#[tauri::command]
fn get_album_cover(album_id: u32) -> Result<String, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    for songs in cache.values() {
        for song in songs {
            if song.al.id == album_id {
                // 使用 match 语句来处理 Option
                match Tag::new().read_from_path(&song.src).unwrap().album_cover() {
                    Some(cover) => {
                        // 现在我们可以安全地访问 cover 的 data 字段

                        return Ok(base64::encode(cover.data));
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

// 加载音乐缓存
fn load_cache_from_disk() -> Result<(), String> {
    if let Ok(file) = std::fs::File::open("music_cache.json") {
        let cache: HashMap<PathBuf, Vec<Song>> = serde_json::from_reader(file).map_err(|e| e.to_string())?;
        let mut cached = MUSIC_CACHE.lock().unwrap();
        *cached = cache;
        Ok(())
    } else {
        Err("Failed to open music cache file".into())
    }
}
// use std::fs;
#[tauri::command]
async fn get_music_file(song_id: u32) -> Result<String, String> {
    println!("Searching for song with ID: {}", song_id);

    // 查找具有匹配 song_id 的歌曲，并立即释放锁
    let song_path = {
        let cache = MUSIC_CACHE.lock().unwrap();
        println!("Cache locked, searching for song...");
        cache.values().flatten().find_map(|s| if s.id == song_id { Some(s.src.clone()) } else { None })
    };

    // 根据找到的路径读取文件
    if let Some(song_path) = song_path {
        println!("Song found, trying to read file: {}", song_path.display());

        // 读取歌曲文件内容
        match async_fs::read(song_path).await {
            Ok(data) => {
                println!("Song finished reading, encoding to Base64");
                Ok(base64::encode(data))
            },
            Err(e) => Err(format!("Failed to read music file: {}", e)),
        }
    } else {
        println!("Music file not found in cache");
        Err("Music file not found".into())
    }
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
    
    // save_music_dirs_to_disk();
    // save_cache_to_disk();
    Ok(())
}


fn save_music_dirs_to_disk() -> Result<(), String> {
    let dirs = MUSIC_DIRS.lock().unwrap();
    let file = std::fs::File::create("music_dirs.json").map_err(|e| e.to_string())?;
    serde_json::to_writer(file, &*dirs).map_err(|e| e.to_string())
}

// 从磁盘加载音乐目录
fn load_music_dirs_from_disk() -> Result<(), String> {
    let file_path = "music_dirs.json";
    let mut file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            // 如果文件不存在或无法打开，则创建新文件
            let audio_dir = dirs::audio_dir().ok_or("No default audio directory found")?;
            let mut file = fs::File::create(file_path).map_err(|e| e.to_string())?;
            let dirs = vec![audio_dir];
            serde_json::to_writer(&mut file, &dirs).map_err(|e| e.to_string())?;
            return Ok(()); // 文件已创建并写入，返回成功
        }
    };

    // 如果文件存在，读取内容
    let mut contents = String::new();
    io::Read::read_to_string(&mut file, &mut contents).map_err(|e| e.to_string())?;
    let dirs: Vec<PathBuf> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
    *MUSIC_DIRS.lock().unwrap() = dirs;
    Ok(())
}


#[tauri::command]
fn add_music_dirs(new_dirs: Vec<String>) -> Result<(), String> {
    let mut music_dirs = MUSIC_DIRS.lock().unwrap();
    music_dirs.extend(new_dirs.iter().map(PathBuf::from));
    Ok(())
}
// 在应用程序的其他部分（例如，在 Tauri 的某个事件处理器中或在初始化时），处理缓存刷新
fn handle_cache_refresh() -> Result<(), String> {
    refresh_music_cache() // 已经返回 Result<(), String>
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
            refresh_music_cache
            ])        
            .setup(|app| {
                // 在应用程序启动时处理缓存刷新
                if let Err(e) = handle_cache_refresh() {
                    eprintln!("Failed to refresh music cache during setup: {}", e);
                }
                Ok(())
            })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
 