import { invoke } from '@tauri-apps/api/core';
import app from '../main.js';

// 使用 Map 替代对象提高缓存管理效率
const objectURLCache = new Map();
const objectURLCounter = new Map();
const requestCache = new Map();
// 应用本地数据缓存结构
const onCacheUpdateListeners = new Map();
const appLocalDataCache = {
    musicList: {
        lastUpdateTimestamp: 0,
        data: []
    },
    folders: {
        lastUpdateTimestamp: 0,
        data: []
    },
    albums: {
        lastUpdateTimestamp: 0,
        data: []
    },
    artists: {
        lastUpdateTimestamp: 0,
        data: []
    },
};
// 统一状态更新方法
const updateAppLocalData = (path, data) => {
    if (appLocalDataCache[path] !== undefined) {
        appLocalDataCache[path].data = data;
        appLocalDataCache[path].lastUpdateTimestamp = Date.now();
        onCacheUpdateListeners.get(path)?.forEach(callback => callback(data));
    } else {
        console.error(`Invalid cache path: ${path}`);
    }
};
const setObjectURL = (id, objectURL) => {
    if (objectURLCache.has(id)) {
        objectURLCounter.set(id, objectURLCounter.get(id) + 1);
        return objectURLCache.get(id);
    }

    objectURLCache.set(id, objectURL);
    objectURLCounter.set(id, 1);
    return objectURL;
};

const getObjectURL = (id) => {
    if (!objectURLCache.has(id)) {
        throw new Error(`Object URL for ${id} is not available.`);
    }

    objectURLCounter.set(id, objectURLCounter.get(id) + 1);
    return objectURLCache.get(id);
};


const isObjectURLAvailable = (id) => {
    return objectURLCache[id] !== undefined;
};


const destroyObjectURL = (id) => {
    if (!objectURLCache.has(id)) return;

    const count = objectURLCounter.get(id) - 1;
    objectURLCounter.set(id, count);

    if (count <= 0) {
        URL.revokeObjectURL(objectURLCache.get(id));
        objectURLCache.delete(id);
        objectURLCounter.delete(id);
    }
};
//  

// 增强型请求处理器
const handleAPIRequest = async (key, invokeFunction, params) => {
    if (objectURLCache.has(key)) {
        return {
            objectURL: getObjectURL(key),
            destroyObjectURL: () => destroyObjectURL(key)
        };
    }

    if (requestCache.has(key)) {
        return requestCache.get(key);
    }

    const requestPromise = invoke(invokeFunction, params)
        .then(data => {
            const objectURL = URL.createObjectURL(new Blob([data]));
            setObjectURL(key, objectURL);
            requestCache.delete(key);
            return { objectURL, destroyObjectURL: () => destroyObjectURL(key) };
        })
        .catch(error => {
            requestCache.delete(key);
            throw error;
        });

    requestCache.set(key, requestPromise);
    return requestPromise;
};
// 使用全大写命名常量
const RESOLUTIONS = {
    ORIGIN: 0,
    MIN: 46*2,
    NORMAL: 46 * 8,
    HIGH: 1024
};

// 参数校验方法
const validateResolution = (resolution) => {
    if (typeof resolution === 'string') {
        if (!(resolution in RESOLUTIONS)) {
            throw new Error(`Invalid resolution string: ${resolution}`);
        }
        return RESOLUTIONS[resolution];
    }
    return resolution;
};

const initApplication =async ()=>{
    console.log('initApplication');
    return  await invoke('init_application');
}
// 导出API函数
export default {
    // 获取音乐列表
    initApplication,
    onCacheUpdate: (path,callback) => {
        let listeners = [];
        if (onCacheUpdateListeners.has(path)) {
            listeners = onCacheUpdateListeners.get(path);
        }
        onCacheUpdateListeners.set(path, [...listeners, callback]);
    },
    getMusicList: async () => {
        let result = await invoke("get_music_list")
        updateAppLocalData('musicList', result);

        return result;
    },

    enum_resolutions: RESOLUTIONS,
    RESOLUTIONS,

    // 获取专辑封面

    // 封面获取逻辑
    getAlbumCover: async (albumId, maxResolution = RESOLUTIONS.NORMAL) => {
        if (albumId < 0) return { objectURL: '', destroyObjectURL: () => { } };

        const resolution = validateResolution(maxResolution);

        const key = resolution === RESOLUTIONS.ORIGIN
            ? `al_${albumId}`
            : `al_${albumId}_${resolution}`;

        const command = resolution === RESOLUTIONS.ORIGIN
            ? "get_album_cover"
            : "get_low_quality_album_cover";

        return handleAPIRequest(
            key,
            command,
            { albumId, ...(resolution > 0 && { maxResolution: resolution }) }
        );
    },


    // 获取专辑封面
    getOriginAlbumCover: async (albumId) => {
        if (albumId < 0) {
            return { objectURL: '', destroyObjectURL: () => { } };
        }
        const albumPicObjectURL_key = `al_${albumId}`;
        return await handleAPIRequest(albumPicObjectURL_key, "get_album_cover", { albumId });
    },

    // 获取音乐文件
    getMusicFile: (songId) =>
        handleAPIRequest(`mf_${songId}`, "get_music_file", { songId }),


    // 简化后的数据获取方法
    getAllMusicDirs: async () => {
        const result = await invoke("get_all_music_dirs");
        updateAppLocalData('folders', result);
        return result;
    },

    addMusicDirs: (dirs) => invoke("add_music_dirs", { newDirs: Array.isArray(dirs) ? dirs : [dirs] }),
    removeMusicDirs: (dirs) => invoke("remove_music_dirs", { dirsToRemove: Array.isArray(dirs) ? dirs : [dirs] }),
    // 刷新音乐缓存
    refreshMusicCache: async () => {
        await invoke("refresh_music_cache");


        // 获取音乐列表
        let musicList = await invoke("get_music_list");

        if (musicList.length === 0) {
            await initApplication();
            musicList = await invoke("get_music_list");
        }

        // 更新本地缓存
        updateAppLocalData('musicList', musicList);
        updateAppLocalData('folders', await invoke("get_all_music_dirs"));
        updateAppLocalData('albums', await invoke("get_all_my_albums"));
        updateAppLocalData('artists', await invoke("get_all_my_artists"));
    },

    // 获取所有专辑
    getAlbums: async () => {
        const result = await invoke("get_all_my_albums");
        updateAppLocalData('albums', result);
        return result;
    },


    // 获取所有艺术家
    getArtists: async () => {
        const result = await invoke("get_all_my_artists");
        updateAppLocalData('artists', result);
        return result;
    },

    // 通过ID获取专辑
    getAlbumById: (albumId) =>
        invoke("get_album_by_id", { albumId: Number(albumId) }),

    // 通过ID获取艺术家
    getArtistById: (artistId) =>
        invoke("get_artist_by_id", { artistId: Number(artistId) }),

    // 通过专辑ID获取歌曲
    getAlbumsSongsById: (albumId) =>
        invoke("get_albums_songs_by_id", { albumId: Number(albumId) }),

    // 通过艺术家ID获取歌曲
    getArtistsSongsById: (artistId) =>
        invoke("get_artists_songs_by_id", { artistId: Number(artistId) }),

    // 销毁对象URL
    destroyObjectURL,
    // 暴露可读写缓存结构
    get appLocalCache() {
        return appLocalDataCache;
    },
    // 新增缓存写入方法（可选）
    setAppLocalCache(path, data) {
        if (appLocalDataCache[path]) {
            appLocalDataCache[path] = {
                ...appLocalDataCache[path],
                ...data,
                lastUpdateTimestamp: Date.now()
            };
        }
    },
    /**
     * Adds a user's music directory.
     * @param {string} dir - The directory to add.
     * @returns {Promise<any>} - A promise that resolves when the directory is added.
     */
    addUsersMusicDir: (dir) => invoke("add_users_music_dir", { dir }),
    getUsersMusicDir: () => invoke("get_users_music_dir")
}
