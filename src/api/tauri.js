import { invoke } from '@tauri-apps/api/core';
let objectURLCache = {};
let objectURLCounter = {};
let requestCache = {};

const setObjectURL = (id, objectURL) => {
    if (isObjectURLAvailable(id)) {
        if (objectURLCounter[id] === undefined) {
            objectURLCounter[id] = 0;
        }
        objectURLCounter[id]++;
        return objectURLCache[id];
    }
    objectURLCache[id] = objectURL;
    objectURLCounter[id] = 1;
    return objectURL;
};

const getObjectURL = (id) => {
    if (!isObjectURLAvailable(id)) {
        throw new Error(`Object URL for ${id} is not available.`);
    }
    if (objectURLCounter[id] === undefined) {
        objectURLCounter[id] = 0;
    }
    objectURLCounter[id]++;
    return objectURLCache[id];
};

const isObjectURLAvailable = (id) => {
    return objectURLCache[id] !== undefined;
};

const destroyObjectURL = (id) => {
    if (!isObjectURLAvailable(id)) {
        return;
    }
    if (objectURLCounter[id] === undefined) {
        objectURLCounter[id] = 0;
    }
    objectURLCounter[id]--;
    if (objectURLCounter[id] <= 0) {
        URL.revokeObjectURL(objectURLCache[id]);
        objectURLCache[id] = undefined;
        objectURLCounter[id] = undefined;
    }
};
//  
const handleAPIRequest = async (key, invokeFunction, params) => {
    if (isObjectURLAvailable(key)) {
        return {
            objectURL: getObjectURL(key),
            destroyObjectURL: () => destroyObjectURL(key)
        };
    }

    if (requestCache.hasOwnProperty(key) && requestCache[key] !== undefined) {
        return requestCache[key];
    }

    const requestPromise = invoke(invokeFunction, params)
        .then((data) => {
            const objectURL = URL.createObjectURL(new Blob([data]));
            setObjectURL(key, objectURL);
            requestCache[key] = undefined;
            return {
                objectURL,
                destroyObjectURL: () => destroyObjectURL(key)
            };
        })
        .catch((error) => {
            requestCache[key] = undefined;
            throw error;
        });

    requestCache[key] = requestPromise;
    return requestPromise;
};

// 枚举分辨率
const enum_resolutions = {
    origin: 0,
    min: 46,
    normal: 46 * 4,
    high: 1024,
}

// 导出API函数
export default {
    // 获取音乐列表
    getMusicList: async () => await invoke("get_music_list"),
    
    enum_resolutions,

        // 获取专辑封面
        getAlbumCover: async (albumId, maxResolution = enum_resolutions.normal) => {
            // 如果专辑ID小于0，返回空对象URL
            if (albumId < 0) {
                return { objectURL: '', destroyObjectURL: () => {} };
            }
            // 如果maxResolution是字符串，转换为对应的枚举值
            if(typeof maxResolution === 'string') {
                maxResolution = enum_resolutions[maxResolution];
            }
            // 如果maxResolution是枚举值origin，直接获取原图
            if (maxResolution == enum_resolutions.origin) {
                const albumPicObjectURL_key = `al_${albumId}`;
                return await handleAPIRequest(albumPicObjectURL_key, "get_album_cover", { albumId });
            }
            // 如果typeof maxResolution是Number，按照逻辑获取
            const albumPicObjectURL_key = `al_${albumId}_${maxResolution}`;
            return await handleAPIRequest(albumPicObjectURL_key, "get_low_quality_album_cover", { albumId, maxResolution });
        },

    // 获取专辑封面
    getOriginAlbumCover: async (albumId) => {
        if (albumId < 0) {
            return { objectURL: '', destroyObjectURL: () => {} };
        }
        const albumPicObjectURL_key = `al_${albumId}`;
        return await handleAPIRequest(albumPicObjectURL_key, "get_album_cover", { albumId });
    },
    
    // 获取音乐文件
    getMusicFile: async (songId) => {
        const musicFileObjectURL_key = `mf_${songId}`;
        return await handleAPIRequest(musicFileObjectURL_key, "get_music_file", { songId });
    },

    // 获取所有音乐目录
    getAllMusicDirs: async () => await invoke("get_all_music_dirs"),
    
    // 移除音乐目录
    removeMusicDirs: async (dirs_to_remove) => await invoke("remove_music_dirs", { dirsToRemove: [dirs_to_remove] }),
    
    // 添加音乐目录
    addMusicDirs: async (new_dirs) => await invoke("add_music_dirs", { newDirs: [new_dirs] }),
    
    // 刷新音乐缓存
    refreshMusicCache: async () => await invoke("refresh_music_cache"),
    
    // 获取所有专辑
    getAlbums: async () => await invoke("get_all_my_albums"),
    
    // 获取所有艺术家
    getArtists: async () => await invoke("get_all_my_artists"),
// 通过ID获取专辑
getAlbumById: async (albumId) => await invoke("get_album_by_id", { albumId: Number(albumId) }),

// 通过ID获取艺术家
getArtistById: async (artistId) => await invoke("get_artist_by_id", { artistId: Number(artistId) }),

// 通过专辑ID获取歌曲
getAlbumsSongsById: async (albumId) => await invoke("get_albums_songs_by_id", { albumId: Number(albumId) }),

// 通过艺术家ID获取歌曲
getArtistsSongsById: async (artistId) => await invoke("get_artists_songs_by_id", { artistId: Number(artistId) }),

// 销毁对象URL
destroyObjectURL: destroyObjectURL
}
