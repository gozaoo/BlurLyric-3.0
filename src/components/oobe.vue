<script>
import manager from '../api/manager.js';
/**对于 manager.tauri中提供的办法：
 * import { invoke } from '@tauri-apps/api/core';
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
    initApplication: ()=>{
        return invoke('init_application') 
    },
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
destroyObjectURL: destroyObjectURL,

/**
 * Adds a user's music directory.
 * @param {string} dir - The directory to add.
 * @returns {Promise<any>} - A promise that resolves when the directory is added.
addUsersMusicDir: async (dir) => await invoke("add_users_music_dir", { dir }),
getUsersMusicDir: async () => await invoke("get_users_music_dir"),
}

 */
import app from '../main.js';
/**对于app的数据结构：
 *     data() {
        return {
            leftBarState: 'short',
            audioManager: null,
            musicTrack: templateEmptyMusicTrack,

            musicTrackIndex: 0,
            scrollState: {
                scrollTop: 0,
                scrollSize: 0
            },
            titles: [

            ],
            title: '主页',
            titleOffsetTop: 0,
            transitionNextMusicWorking: false,
            config: {
                language: 'zh_cn',
                audio: {
                    smartStreamAudioList: true,
                    audioStreamDuration: 7, // Unit: second
                    audioStateHandlerTPS: 20
                },
                ui: {
                    musicDetailFontScale: 1,
                    musicDetailFontSizeAdaptive: true,
                    lyricComponentStyle: 'normal', // [normal, spawnWordByWord]
                    dynamicBackground: false,
                    wordByWordLyrics: true,
                    lyricBlurEffect: false,
                    lyricAnimationType: 'spring', // [spring, cubic_bezier, linear]
                    lyricAnimationPreset: {
                        spring: {
                            mass: 1,
                            stiffness: 95,
                            damping: 14.5
                        },
                        cubic_bezier: [
                            [.3, .7],
                            [.2, 1]
                        ]
                    },
                    lyricScrollDelayPropagation: true,
                    lyricScrollDelayAmount: 50, // ms
                }
            },
            audioState: ref({
                error_onloadSrc: false,
                loading: false,
                canplay: false,
                currentTime: 0,
                currentTime_round: 0,
                duration: 0,
                duration_round: 0,
                playing: false,
                volume: 1
            }),
            trackState: {
                playMode: 'loopPlaylist',
                allPlayModes: ['loopPlaylist', 'loopSingle', 'stopAfterSingle', 'randomPlay', 'smartRecommend']
            },
            appState: {
                runOnTauri: (window.__TAURI_INTERNALS__) ? true : false,
                screenType: null, // ['landscape','portrai,'mini']
            },
            source: {
                local: {
                    lastUpdateTimestamp: 0,
                    musicList: [],
                    folders: [],
                    albums: [],
                    artists: [],
                },
                online: [{
                    name: 'API1',
                    type: 'NeteaseCloudMusicApi',
                    apiUrl: 'http://localhost:3000/'
                }]
            },
            resizeEvent: {},

            messageList: []
        }
    },
 */
export default {
    data() {
        return {
            allMusicDirs: [],
            allMusicList: [],
            step: 0,
            /**
             * 0: 拉取缓存
             * 1. 引导增加目录
             * 2: 扫描音乐
             * 3. 检查完毕
             */
        }
    },
    async mounted() {
        this.allMusicList = await manager.tauri.getMusicList();
        this.getAllMusicDirs = await manager.tauri.getAllMusicDirs();

        // 如果均为空，则尝试初始化
        if (this.allMusicList.length === 0 && this.allMusicDirs.length === 0) {
            await manager.tauri.initApplication();
        }

        // 如果仍为空，则...
        if (this.allMusicDirs.length === 0) {
            this.step = 1;
        }
        this.step = 3;
    },
    watch:{
        step:{
            handler(_result, __){
                if(_result === 2){
                    this.refershMusicCache();
                }
            },
            // immediate: true,
        }
    }
}
</script>
<template>
    <div v-show="step != 3" class="oobeContainer">
        <div v-if="step === 0">
            <h1>正在初始化</h1>
            <p>请稍等</p>
        </div>
        <div v-if="step === 1">
            <h1>欢迎使用</h1>
            <p>这是一款开源的音乐播放器</p>
            <p>你可以在这里管理你的音乐</p>
            <p>首先，我们需要知道你的音乐在哪里</p>
        </div>
    </div>
</template>
<style scoped>
    .oobeContainer {
        position: fixed;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
        width: 100vw;
        background: var(--background-bar);
        left: 0;
        visibility: visible;
        opacity: 1;
        transition: 1s;
        top: 0;
        right: 0;
        bottom: 0;
        z-index: 999;
    }
    .oobeContainer>div{
        padding: 10px 20px 40px 20px ;
        width: 200px;
        background-color: #fff;
        border-radius: 7px;
        box-shadow: var(--Shadow-value-offsetY-high);
    }
</style>