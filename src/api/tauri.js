// 当使用 Tauri API 的 npm 包时
import {
    invoke
} from '@tauri-apps/api/core';
// import { open,BaseDirectory  } from "@tauri-apps/plugin-fs";

let objectURLCache = {};
let objectURLCounter = {};

const setObjectURL = (id, objectURL) => {
    if (isObjectURLAvalible(id)) {

        objectURLCounter[id]++;

        return objectURLCache[id];
    }

    objectURLCache[id] = objectURL;
    objectURLCounter[id] = 1;
    return objectURL;
};

const getObjectURL = (id) => {
    // console.log('[reuse]id:', id);

    objectURLCounter[id]++;
    return objectURLCache[id];
}
const isObjectURLAvalible = (id) => {
    return objectURLCache[id] != undefined;
};

const destroyObjectURL = (id) => {
    if (objectURLCounter[id]) {
        objectURLCounter[id]--;
        if (objectURLCounter[id] === 0) {
            // console.log('[destory]id:', id);
            objectURLCache[id] = undefined;
            objectURLCounter[id] = undefined;
            URL.revokeObjectURL(objectURLCache[id]);

        }
    }
};

export default {
    getMusicList: async () => await invoke("get_music_list"),
    getAlbumCover: async (albumId) => {
        let objectURL
        if (albumId < 0) {
            return ''
        };
        const albumPicObjectURL_key = `al_${albumId}`;
        // 如果已经有了，直接返回
        if (isObjectURLAvalible(albumPicObjectURL_key)) {

            return {
                objectURL: getObjectURL(albumPicObjectURL_key),
                destroyObjectURL: () => destroyObjectURL(albumPicObjectURL_key)
            }
        }
        // 如果没有，请求后返回
        await invoke("get_album_cover", {
                albumId
            })
            .then((albumPicData) => {
                if (!isObjectURLAvalible(albumPicObjectURL_key)) {
                    objectURL = URL.createObjectURL(new Blob([albumPicData]));
                    objectURL = setObjectURL(albumPicObjectURL_key, objectURL);
                } else {
                    objectURL = getObjectURL(albumPicObjectURL_key);
                }
            })
        return {
            objectURL,
            destroyObjectURL: () => destroyObjectURL(albumPicObjectURL_key)
        }


    },
    getMusicFile: async (songId) => {
        let objectURL
        const musicFileObjectURL_key = `mf_${songId}`;

        if (isObjectURLAvalible(musicFileObjectURL_key)) {
            return {
                objectURL: getObjectURL(musicFileObjectURL_key),
                destroyObjectURL: () => destroyObjectURL(musicFileObjectURL_key)
            }
        }
        await invoke("get_music_file", {
                songId
            })
            .then((audioData) => {
                if (!isObjectURLAvalible(musicFileObjectURL_key)) {
                    objectURL = setObjectURL(musicFileObjectURL_key, URL.createObjectURL(new Blob([audioData])));
                    objectURL = setObjectURL(musicFileObjectURL_key, objectURL);
                } else {
                    objectURL = getObjectURL(musicFileObjectURL_key);
                }
            });
        return {
            objectURL,
            destroyObjectURL: () => destroyObjectURL(musicFileObjectURL_key)
        };
    },
    getAllMusicDirs: async () => await invoke("get_all_music_dirs"),
    removeMusicDirs: async (dirs_to_remove) => await invoke("remove_music_dirs", {
        dirsToRemove: [dirs_to_remove]
    }),
    addMusicDirs: async (new_dirs) => await invoke("add_music_dirs", {
        newDirs: [new_dirs]
    }),
    refreshMusicCache: async () => await invoke("refresh_music_cache"),
    getAlbums: async () => await invoke("get_all_my_albums"),
    getArtists: async () => await invoke("get_all_my_artists"),
<<<<<<< Updated upstream
    getAlbumById: async (albumId) => await invoke("get_album_by_id", {
        albumId: Number(albumId)
    }),
    getArtistById: async (artistId) => await invoke("get_artist_by_id", {
        artistId: Number(artistId)
    }),
    getAlbumsSongsById: async (albumId) => await invoke("get_albums_songs_by_id", {
        albumId: Number(albumId)
    }),
    getArtistsSongsById: async (artistId) => await invoke("get_artists_songs_by_id", {
        artistId: Number(artistId)
    }),
    destroyObjectURL: destroyObjectURL
};
=======
    getAlbumById: async (albumId) => await invoke("get_album_by_id", { albumId: Number(albumId) }),
    getArtistById: async (artistId) => await invoke("get_artist_by_id", { artistId: Number(artistId) }),
    getAlbumsSongsById: async (albumId) => await invoke("get_albums_songs_by_id", { albumId: Number(albumId) }),
    closeApp: invoke("close_app"),
    getArtistsSongsById: async (artistId) => await invoke("get_artists_songs_by_id", { artistId: Number(artistId) })
}
>>>>>>> Stashed changes
