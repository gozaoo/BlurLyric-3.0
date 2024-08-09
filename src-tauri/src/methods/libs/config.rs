use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::models::music_info::SingleMusic;
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    folder_path: Vec<PathBuf>,
    folders: Vec<Folder>,
    folder_info: Vec<FolderInfo>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FolderInfo {  
    path: PathBuf,
    music: Vec<SingleMusic>,
}

pub fn read_config<T>(file_path: PathBuf, on_non_exist: fn() -> T) -> T
where
    T: serde::de::DeserializeOwned,
{
    if file_path.exists() {
        let config_str = fs::read_to_string(file_path).expect("Failed to read config file");
        serde_json::from_str(&config_str).expect("Failed to parse config")
    } else {
        on_non_exist()
    }
}

pub fn operate_config<T>(file_path: PathBuf, on_non_exist: fn() -> T, write: fn(T) -> T)
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let mut config = read_config(file_path.clone(), on_non_exist);
    config = write(config);
    let config_str = serde_json::to_string(&config).expect("Failed to serialize config");
    fs::write(file_path, config_str).expect("Failed to write config file");
}
