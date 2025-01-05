use audiotags::Tag;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as serde_json_value};

use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::fs as async_fs;

lazy_static! {
    // ID 计数器
    static ref SONG_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref ARTIST_ID_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref ALBUM_ID_COUNTER: Mutex<u32> = Mutex::new(0);

    // 音乐、艺人、专辑的缓存
    static ref MUSIC_CACHE: Mutex<HashMap<PathBuf, Vec<Song>>> = Mutex::new(HashMap::new());
    static ref ARTIST_CACHE: Mutex<HashMap<String, Artist>> = Mutex::new(HashMap::new());
    static ref ALBUM_CACHE: Mutex<HashMap<String, Album>> = Mutex::new(HashMap::new());

    // 艺人相关缓存
    static ref ARTIST_SONGS_MAP: Mutex<HashMap<u32, Vec<Song>>> = Mutex::new(HashMap::new());
    static ref ALBUM_SONGS_MAP: Mutex<HashMap<u32, Vec<Song>>> = Mutex::new(HashMap::new());
    static ref MUSIC_DIRS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
}

fn get_or_create_artist(name: String) -> Artist {
    let mut cache = ARTIST_CACHE.lock().unwrap();
    cache
        .entry(name.clone())
        .or_insert_with(|| {
            let id = next_id(&ARTIST_ID_COUNTER);
            Artist {
                id,
                name: name.clone(),
                alias: vec![],
            }
        })
        .clone()
}

fn get_or_create_album(name: String) -> Album {
    let mut cache = ALBUM_CACHE.lock().unwrap();
    cache
        .entry(name.clone())
        .or_insert_with(|| {
            let id = next_id(&ALBUM_ID_COUNTER);
            Album {
                id,
                name: name,
                pic_url: String::new(),
            }
        })
        .clone()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Song {
    name: String,
    id: u32,
    ar: Vec<Artist>,
    lyric: String,
    al: Album,
    src: PathBuf,
    track_number:u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Artist {
    id: u32,
    name: String,
    alias: Vec<String>,
}

impl Artist {
    fn get_songs(&self) -> Vec<Song> {
        let map = ARTIST_SONGS_MAP.lock().unwrap();
        map.get(&self.id).unwrap_or(&vec![]).clone()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[tauri::command]
fn get_all_my_albums() -> Result<Vec<Album>, String> {
    let album_cache = ALBUM_CACHE.lock().unwrap();
    let albums = album_cache.values().cloned().collect();
    Ok(albums)
}

#[tauri::command]
fn get_all_my_artists() -> Result<Vec<Artist>, String> {
    let artist_cache = ARTIST_CACHE.lock().unwrap();
    let artists = artist_cache.values().cloned().collect();
    Ok(artists)
}
// Tauri命令
#[tauri::command]
fn get_music_list() -> Result<Vec<Song>, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    Ok(cache
        .values()
        .flat_map(|songs| songs.iter().cloned())
        .collect())
}
#[tauri::command]
fn get_artist_by_id(artist_id: u32) -> Result<Artist, String> {
    let artist_cache = ARTIST_CACHE.lock().unwrap();
    artist_cache
        .values()
        .find(|artist| artist.id == artist_id)
        .cloned()
        .ok_or_else(|| "Artist not found".to_string())
}

#[tauri::command]
fn get_album_by_id(album_id: u32) -> Result<Album, String> {
    let album_cache = ALBUM_CACHE.lock().unwrap();
    album_cache
        .values()
        .find(|album| album.id == album_id)
        .cloned()
        .ok_or_else(|| "Album not found".to_string())
}

#[tauri::command]
fn get_artists_songs_by_id(artist_id: u32) -> Result<Vec<Song>, String> {
    let artist_songs_map = ARTIST_SONGS_MAP.lock().unwrap();
    Ok(artist_songs_map.get(&artist_id).cloned().unwrap())
}

#[tauri::command]
fn get_albums_songs_by_id(album_id: u32) -> Result<Vec<Song>, String> {
    let album_songs_map = ALBUM_SONGS_MAP.lock().unwrap();
    Ok(album_songs_map.get(&album_id).cloned().unwrap())
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
fn split_artist_names(artists: Vec<&str>) -> Vec<&str> {
    let mut split_names = Vec::new();
    for name in artists {
        // 检查并分割特定字符
        let parts: Vec<&str> = name
            .split('/')
            .flat_map(|part| part.split('&'))
            .flat_map(|part| part.split('\\'))
            .filter(|part| !part.is_empty()) // 过滤掉空字符串
            .collect();
        split_names.extend(parts);
    }
    split_names
}

// 假设Song, Artist, Album, Tag, next_id和SONG_ID_COUNTER等都已经定义
fn parse_music_file(file: PathBuf) -> Result<Song, String> {
    // 尝试读取标签
    let tag_result = Tag::new().read_from_path(&file);

    // 获取文件名作为备用标题
    let file_name = file
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown Title")
        .to_string();

    // 根据标签读取结果处理
    let song_data = match tag_result {
        Ok(tag) => {
            // 如果成功读取标签，则使用标签中的信息
            
            let track_number = tag.track_number().unwrap_or_else(|| 0);
            let title = tag
                .title()
                .map(|s| s.to_string())
                .unwrap_or_else(|| file_name.clone());
            // 处理多个艺术家
            let artists = split_artist_names(tag.artists().unwrap_or(vec![&"Unknown Artist"]))
                .iter()
                .map(|name| {
                    // println!("{}",name);
                    get_or_create_artist(name.to_string())
                })
                .collect::<Vec<Artist>>();

            let album_name = tag
                .album_title()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unknown Album".to_string());
            
            // let lyrics = 
            let album = get_or_create_album(album_name);
            (title, artists, album,track_number)
        }
        Err(_) => {
            // 如果读取标签失败，则使用默认值
            (
                file_name.clone(),
                vec![get_or_create_artist("Unknown Artist".to_string())],
                get_or_create_album("Unknown Album".to_string()),
                0,
                // "No lyrics found".to_string()
            )
        }
    };

    let song = Song {
        name: song_data.0,
        id: next_id(&SONG_ID_COUNTER),
        ar: song_data.1,
        lyric: String::new(),
        al: song_data.2,
        src: file,
        track_number: song_data.3
    };
    {
        let mut artist_songs_map = ARTIST_SONGS_MAP.lock().unwrap();
        let mut album_songs_map = ALBUM_SONGS_MAP.lock().unwrap();

        for artist in &song.ar {
            artist_songs_map
                .entry(artist.id)
                .or_insert_with(Vec::new)
                .push(song.clone());
        }

        album_songs_map
            .entry(song.al.id)
            .or_insert_with(Vec::new)
            .push(song.clone());
    }
    Ok(song)
}

// 缓存音乐列表
fn cache_music_list(dir: PathBuf, songs: Vec<Song>) {
    MUSIC_CACHE.lock().unwrap().insert(dir, songs);
}

#[tauri::command]
fn refresh_music_cache() -> Result<(), String> {
    // 重置ID计数器
    *SONG_ID_COUNTER.lock().unwrap() = 0;
    *ARTIST_ID_COUNTER.lock().unwrap() = 0;
    *ALBUM_ID_COUNTER.lock().unwrap() = 0;

    // 清空音乐、艺术家和专辑的缓存
    MUSIC_CACHE.lock().unwrap().clear();
    ARTIST_CACHE.lock().unwrap().clear();
    ALBUM_CACHE.lock().unwrap().clear();
    ARTIST_SONGS_MAP.lock().unwrap().clear();
    ALBUM_SONGS_MAP.lock().unwrap().clear();

    let music_dirs = MUSIC_DIRS.lock().unwrap();
    let mut new_cache = HashMap::new();

    for dir in &*music_dirs {
        if dir.is_dir() {
            let songs = scan_music_files(dir);
            let parsed_songs: Result<Vec<Song>, String> =
                songs.into_iter().map(parse_music_file).collect();
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
    fn to_json(&self) -> serde_json_value {
        json!({
            "name": self.name,
            "id": self.id,
            "ar": self.ar.iter().map(|ar| {
                json!({
                    "id": ar.id,
                    "name": ar.name,
                    "alias": ar.alias,
                })
            }).collect::<Vec<serde_json_value>>(),
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
fn get_album_cover(album_id: u32) -> Result<tauri::ipc::Response, String> {
    let cache = MUSIC_CACHE.lock().unwrap();
    for songs in cache.values() {
        for song in songs {
            if song.al.id == album_id {
                // 使用 match 语句来处理 Option
                match Tag::new().read_from_path(&song.src).unwrap().album_cover() {
                    Some(cover) => {
                        let cover_data: Vec<u8> = cover.data.to_vec();

                        // 现在我们可以安全地将 Vec<u8> 转换为 InvokeResponseBody
                        return Ok(tauri::ipc::Response::new(cover_data));
                    }
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

// 加载音乐缓存
// fn load_cache_from_disk() -> Result<(), String> {
//     let file_path = "music_cache.json";
//     let file = match std::fs::File::open(file_path) {
//         Ok(file) => file,
//         Err(_) => {
//             // 如果文件不存在，初始化一个空的缓存
//             let empty_cache: HashMap<PathBuf, Vec<Song>> = HashMap::new();
//             let file = std::fs::File::create(file_path).map_err(|e| e.to_string())?;
//             serde_json::to_writer(&file, &empty_cache).map_err(|e| e.to_string())?;
//             return Ok(()); // 初始化完成，返回成功
//         }
//     };

//     // 如果文件存在，读取内容
//     let cache: HashMap<PathBuf, Vec<Song>> =
//         serde_json::from_reader(file).map_err(|e| e.to_string())?;
//     let mut cached = MUSIC_CACHE.lock().unwrap();
//     *cached = cache;
//     Ok(())
// }
#[tauri::command]
async fn get_music_file(song_id: u32) -> Result<tauri::ipc::Response, String> {
    println!("Searching for song with ID: {}", song_id);

    // 查找具有匹配 song_id 的歌曲，并立即释放锁
    let song_path = {
        let cache = MUSIC_CACHE.lock().unwrap();
        println!("Cache locked, searching for song...");
        cache.values().flatten().find_map(|s| {
            if s.id == song_id {
                Some(s.src.clone())
            } else {
                None
            }
        })
    };

    // 根据找到的路径读取文件
    if let Some(song_path) = song_path {
        println!("Song found, trying to read file: {}", song_path.display());

        // 读取歌曲文件内容
        match async_fs::read(song_path).await {
            Ok(data) => {
                println!("Song finished reading, sending to front");
                Ok(tauri::ipc::Response::new(data))
            }
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
    let dirs = cache
        .keys()
        .map(|path| path.display().to_string())
        .collect();
    Ok(dirs)
}
#[tauri::command]
fn remove_music_dirs(dirs_to_remove: Vec<String>) -> Result<(), String> {
    {
        let mut music_dirs = MUSIC_DIRS.lock().unwrap();
        music_dirs.retain(|dir| !dirs_to_remove.contains(&dir.display().to_string()));

        let mut cache = MUSIC_CACHE.lock().unwrap();
        for dir_str in dirs_to_remove {
            let dir = PathBuf::from(dir_str);
            cache.remove(&dir);
        }
    }

    save_music_dirs_to_disk();
    Ok(())
}

fn save_music_dirs_to_disk() -> Result<(), String> {
    let dirs_clone = {
        println!("Locking MUSIC_DIRS");
        let dirs = MUSIC_DIRS.lock().unwrap();
        println!("Locked MUSIC_DIRS and cloned the contents.");
        dirs.clone()
    };
    let file = std::fs::File::create("music_dirs.json").map_err(|e| e.to_string())?;
    serde_json::to_writer(file, &dirs_clone).map_err(|e| e.to_string())
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
    // 锁定互斥锁，更新内容，然后立即释放锁
    let mut music_dirs = MUSIC_DIRS.lock().unwrap();
    *music_dirs = dirs;
    Ok(())
}

#[tauri::command]
fn add_music_dirs(new_dirs: Vec<String>) -> Result<(), String> {
    {
        let mut music_dirs = MUSIC_DIRS.lock().unwrap();
        music_dirs.extend(new_dirs.iter().map(PathBuf::from));
    }
    save_music_dirs_to_disk();
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
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_music_list,
            get_all_music_dirs,
            add_music_dirs,
            remove_music_dirs,
            get_album_cover,
            get_music_file,
            refresh_music_cache,
            get_all_my_albums,
            get_all_my_artists,
            get_album_by_id,
            get_artist_by_id,
            get_albums_songs_by_id,
            get_artists_songs_by_id,
            close_app
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
// 新增的关闭应用的方法
#[tauri::command]
fn close_app(window: tauri::Window) {
    // 关闭当前窗口
    window.close().unwrap();
}