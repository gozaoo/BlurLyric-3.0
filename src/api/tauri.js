// 当使用 Tauri API 的 npm 包时
import { invoke } from '@tauri-apps/api/core';

// 使用命令
export default {
    getMusicList: async () => await invoke("get_music_list"),
    getAlbumCover: async (album_id) => await invoke("get_album_cover", { album_id }),
    getMusicFile: async (song_id) => await invoke("get_music_file", { song_id }),
    getAllMusicDirs: async () => await invoke("get_all_music_dirs"),
    removeMusicDirs: async (dirs_to_remove) => await invoke("remove_music_dirs", {dirsToRemove:[ dirs_to_remove] }),
    addMusicDirs: async (new_dirs) => await invoke("add_music_dirs", { newDirs: [new_dirs] }),
    refreshMusicCache: async () => await invoke("refresh_music_cache")
}