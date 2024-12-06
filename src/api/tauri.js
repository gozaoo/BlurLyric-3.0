// 当使用 Tauri API 的 npm 包时
import { invoke } from '@tauri-apps/api/core';
// import { open,BaseDirectory  } from "@tauri-apps/plugin-fs";
// 使用命令
export default {
    getMusicList: async () => await invoke("get_music_list"),
    getAlbumCover: async (albumId) => {
        let objectURL
        if(albumId<0) {return ''};
        await invoke("get_album_cover", { albumId })            
        .then((albumPicData )=>{
            objectURL = URL.createObjectURL( new Blob([albumPicData]));
        })
        return objectURL
    
    
    },
    getMusicFile: async (songId) =>{
            let objectURL
            await invoke("get_music_file", { songId })
            .then((audioData )=>{
                objectURL = URL.createObjectURL( new Blob([audioData]));
            })
            return objectURL
        },
    getAllMusicDirs: async () => await invoke("get_all_music_dirs"),
    removeMusicDirs: async (dirs_to_remove) => await invoke("remove_music_dirs", {dirsToRemove:[ dirs_to_remove] }),
    addMusicDirs: async (new_dirs) => await invoke("add_music_dirs", { newDirs: [new_dirs] }),
    refreshMusicCache: async () => await invoke("refresh_music_cache")
}