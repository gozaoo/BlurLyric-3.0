use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct SingleMusic {
    name: String,
    artists: Vec<String>,
    id: String,
    path: PathBuf,
    lyrics: String, // 这里简化为字符串，实际情况可能需要更复杂的结构
    album_info: AlbumInfo,
    track: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FolderInfo {  
    path: PathBuf,
    music: Vec<SingleMusic>,
}
#[derive(Debug, Deserialize, Serialize)]

pub struct MusicList {
    id: String,
    path: PathBuf,
    musics: Vec<SingleMusic>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumInfo {
    name: String,
    id: String,
    describe: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MusicInfo {
    name: String,
    artist: String,
    // 其他元数据字段...
}
