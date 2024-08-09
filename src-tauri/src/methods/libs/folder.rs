use std::path::PathBuf;
use audiotags::Tag;
use core::convert::AsRef;
#[derive(Debug)]
pub struct MusicList {
    id: String,
    path: PathBuf,
    musics: Vec<SingleMusic>,
}

#[derive(Debug)]
pub struct SingleMusic {
    name: String,
    artists: Vec<String>,
    id: String,
    path: PathBuf,
    lyrics: String, // 这里简化为字符串，实际情况可能需要更复杂的结构
    album_info: AlbumInfo,
    track: u32,
}

#[derive(Debug)]
pub struct AlbumInfo {
    name: String,
    id: String,
    describe: String,
}
pub struct MusicInfo {
    name: String,
    artist: String,
    // 其他元数据字段...
}



pub fn scan_folder(path: impl AsRef<std::path::Path>) -> Vec<MusicInfo> {
    let mut music_infos = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            if file_name.to_string_lossy().ends_with(".mp3") ||
               file_name.to_string_lossy().ends_with(".flac") ||
               file_name.to_string_lossy().ends_with(".m4a") {
                let mut tag = Tag::new().read_from_path(path.to_str().unwrap()).unwrap();
                // 解析元数据并填充 MusicInfo 结构体
                let music_info = MusicInfo {
                    name: tag.title().unwrap_or("Unknown"),
                    artist: tag.artist().unwrap_or("Unknown"),
                    // 其他元数据字段...
                };
                music_infos.push(music_info);
            }
        }
    }
    music_infos
}
