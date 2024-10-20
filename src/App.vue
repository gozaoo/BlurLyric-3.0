<script>
    import topBar from './components/topBar.vue'
    import rightBlock from './components/rightBlock.vue'
    import leftBar from './components/leftBar.vue'
    import musicInfoPage from './components/musicInfoPage.vue'
    import {
        computed,
        ref,
        onMounted
    } from 'vue'


    export default {
        name: 'app',
        components: {
            topBar,
            leftBar,
            rightBlock,
            musicInfoPage,
        },
        data() {
            return {
                leftBarState: 'short',
                audioManager: null,
                musicTrack: [{
                    name: "请选择您的音乐",
                    id: -2,
                    ar: [{
                        id: -2,
                        name: "享受 BlurLyric 为您带来的舒适体验",
                        alias: []
                    }],
                    lyric: {
                        type: "yrc",
                        lines: [{
                            startTime: 0,
                            duration: 2,
                            endTime: 2,
                            words: [{
                                    startTime: 0,
                                    duration: 1,
                                    endTime: 1,
                                    word: "Hello "
                                },
                                {
                                    startTime: 1,
                                    duration: 0.5,
                                    endTime: 1.5,
                                    word: "World"
                                }
                            ],
                            text: "Hello World"
                        }]
                    },
                    al: {
                        id: -2,
                        name: "",
                        picUrl: "&quot;&quot;",
                        // picUrl: cover,
                    },
                    src: null
                }],

                musicTrackIndex: 0,
                scrollState: {
                    scrollTop: 0,
                    scrollSize: 0
                },
                titles: [

                ],
                title: '主页',
                titleOffsetTop: 0,
                config: {
                    language: 'zh_cn',
                    audio: {
                        smartStreamAudioList: true,
                        audioStreamDuration: 10, // Unit: second
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
                    playing: false
                }),
                trackState: {
                    playMode: 'loopPlaylist',
                    allPlayModes: ['loopPlaylist', 'loopSingle', 'stopAfterSingle', 'randomPlay', 'smartRecommend']
                },
                appState: {
                    isTauri: false,
                    screenType: null, // ['landscape','portrai,'mini']
                },
                source: {
                    local: [],
                    online: [{
                        name: 'API1',
                        type: 'NeteaseCloudMusicApi',
                        apiUrl: 'http://localhost:3000/'
                    }]
                }
            }
        },
        provide() {
            return {
                scrollState: computed(() => this.scrollState),
                leftBarState: computed(() => this.leftBarState),
                config: computed(() => this.config),

                currentMusicInfo: computed(() => this.musicTrack[this.musicTrackIndex]),
                musicTrack: computed(() => this.musicTrack),
                musicTrackIndex: computed(() => this.musicTrackIndex),

                audioManager: computed(() => this.audioManager),
                audioState: computed(() => this.audioState),
                editConfig: this.editConfig,

                setScrollState: this.setScrollState,
                regTitle: this.regTitle,

                trackState: computed(() => this.trackState),
                changePlayMode: this.changePlayMode,
                nextMusic: this.nextMusic,
                prevMusic: this.prevMusic,
                getNextMusicIndex: this.getNextMusicIndex,
                getPrevMusicIndex: this.getPrevMusicIndex,
                appState: computed(() => this.appState),
                source: computed(() => this.source)
            };
        },
        methods: {
            handleResize() {
                const width = window.innerWidth;
                if (width >= 768) { // 假设768px及以上为横屏
                    this.appState.screenType = 'landscape';
                } else if (width >= 480) { // 假设480px到767px为竖屏
                    this.appState.screenType = 'portrait';
                } else { // 小于480px为迷你屏
                    this.appState.screenType = 'mini';
                }
            },
            /**
             * 处理标题和主内容框滚动事件
             * @param text String
             * @param offsetTop Number
             * @param minHiddenTop Number
             */
            regTitle(text, offsetTop, minHiddenTop) {
                let currentIndex = this.title.length;
                this.titles.push({
                    text,
                    offsetTop,
                    minHiddenTop
                });

                // Sort titles array by offsetTop to ensure they are in the correct order
                this.titles.sort((a, b) => a.offsetTop - b.offsetTop);

                this.updateTitle();

                return {
                    cancelReg: () => {
                        const index = this.titles.findIndex(title => title.offsetTop === offsetTop);
                        if (index !== -1) {
                            this.titles.splice(index, 1);
                        }
                    }
                };
            },
            setScrollState(state) {
                this.scrollState = state;
                this.updateTitle();
            },
            updateTitle() {
                let activeTitle = null;
                let titleOffsetTop = 0;

                for (let title of this.titles) {
                    if (title.offsetTop - this.scrollState.scrollTop < title.minHiddenTop) {
                        activeTitle = title;
                        titleOffsetTop = title.offsetTop - this.scrollState.scrollTop;
                    } else {
                        break; // Since titles are sorted by offsetTop, no need to continue once we find a title that doesn't qualify
                    }
                }

                if (activeTitle) {
                    this.title = activeTitle.text;
                    this.titleOffsetTop = titleOffsetTop;
                }
            },

            editConfig(editEvent) {
                this.config = editEvent(this.config)
            },

            audioManagerConstruct(url) {
                const newAudio = document.createElement('audio');
                newAudio.src = url
                this.setupMediaSession();
                const {
                    audioState
                } = this;

                let updateAudioState = (state, value) => {
                    audioState[state] = value;
                };

                let loadeddataHandler = () => {
                    if (newAudio.readyState >= 2) {
                        updateAudioState('canplay', true);
                    }
                    if (newAudio.readyState >= 3) {
                        updateAudioState('duration', newAudio.duration);

                    }
                };

                let playingHandler = () => {
                    updateAudioState('playing', true);
                    currentEventHandler();
                };

                let pauseHandler = () => {
                    updateAudioState('playing', false);
                };

                let timeupdateHandler = () => {
                    updateAudioState('currentTime', newAudio.currentTime);
                };

                let currentEventHandler = () => {
                    if (audioState.playing) {
                        updateAudioState('currentTime_round', Math.trunc(newAudio.currentTime));
                        updateAudioState('duration_round', Math.trunc(newAudio.duration));

                        // 更新时间的频率由 audioStateHandlerTPS 控制
                        setTimeout(() => {
                            timeupdateHandler();
                            currentEventHandler();
                        }, 1000 / this.config.audio.audioStateHandlerTPS);
                    }

                };

                // 检查音频是否可播放并播放
                const checkAndPlay = () => {

                    if (newAudio.readyState >= 4) { // HAVE_ENOUGH_DATA
                        newAudio.play();
                    } else {
                        newAudio.addEventListener('canplay', () => {
                            try {
                                newAudio.play();
                            } catch {

                            }
                        });
                    }
                };

                newAudio.addEventListener('loadeddata', () => {
                    loadeddataHandler()
                });
                newAudio.addEventListener('playing', () => {
                    playingHandler()
                });
                newAudio.addEventListener('pause', () => {
                    pauseHandler()
                });
                newAudio.addEventListener('timeupdate', () => {
                    timeupdateHandler()
                });

                const cancelListener = () => {
                    // 移除所有事件监听
                    newAudio.removeEventListener('loadeddata', () => {
                        loadeddataHandler()
                    });
                    newAudio.removeEventListener('playing', () => {
                        playingHandler()
                    });
                    newAudio.removeEventListener('pause', () => {
                        pauseHandler()
                    });
                    newAudio.removeEventListener('timeupdate', () => {
                        timeupdateHandler()
                    });
                };

                let destroyThisManager = () => {
                    console.log('des');

                    newAudio.pause();
                    cancelListener();
                };

                this.audioManager = {
                    audioDom: newAudio,
                    cancelListener,
                    destroyThisManager: () => {
                        destroyThisManager()
                    },
                    play: checkAndPlay,
                    pause: () => {
                        newAudio.pause()
                    }
                };
            },

            setupMediaSession() {
                if ("mediaSession" in navigator) {
                    const metadata = new MediaMetadata({
                        title: this.currentMusicInfo.name, // 音频标题
                        artist: this.currentMusicInfo.ar.map(artist => artist.name).join('/'), // 艺术家
                        album: this.currentMusicInfo.al.name, // 专辑名称
                        artwork: [{
                            src: this.currentMusicInfo.al.picUrl
                        }] // 专辑封面图片
                    });

                    navigator.mediaSession.metadata = metadata;

                    // 设置媒体会话的播放和暂停事件处理
                    navigator.mediaSession.setActionHandler('play', () => {
                        this.playAudio();
                    });
                    navigator.mediaSession.setActionHandler('pause', () => {
                        this.pauseAudio();
                    });
                }
            },

            playAudio() {
                if (this.checkMusicIsUsable(this.musicTrackIndex) == false) {
                    return
                }

                if (this.audioManager && this.audioManager.audioDom) {
                    if (this.audioState.playing == false) {
                        this.audioManager.play();
                    }
                }
            },

            pauseAudio() {
                this.audioManager.pause();
            },

            changePlayMode() {
                const allPlayModes = this.trackState.allPlayModes
                const playMode = this.trackState.playMode

                let nextIndex = allPlayModes.findIndex((value) => playMode == value) + 1

                if (nextIndex == allPlayModes.length) nextIndex = 0;

                this.trackState.playMode = allPlayModes[nextIndex]

            },
            getNextMusicIndex() {
                switch (this.trackState.playMode) {
                    case 'loopPlaylist':
                        let nextIndex = this.musicTrackIndex + 1
                        if (nextIndex >= this.musicTrack.length) nextIndex = 0
                        return nextIndex

                    default:
                        return 0

                }
            },
            getPrevMusicIndex() {
                switch (this.trackState.playMode) {
                    case 'loopPlaylist':
                        let nextIndex = this.musicTrackIndex - 1
                        if (nextIndex < 0) nextIndex = this.musicTrack.length - 1
                        return nextIndex

                    default:
                        return 0

                }
            },
            nextMusic() {
                this.musicTrackIndex = this.getNextMusicIndex();

                if (this.checkMusicIsUsable(this.musicTrackIndex) == false) {
                    return
                }

                this.audioManager.destroyThisManager()
                this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex].src)
                this.audioManager.play()
            },
            prevMusic() {
                this.musicTrackIndex = this.getPrevMusicIndex();
                if (this.checkMusicIsUsable(this.musicTrackIndex) == false) {
                    return
                }
                this.audioManager.destroyThisManager()
                this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex].src)
                this.audioManager.play()
            },
            checkMusicIsUsable(index) {
                if (this.musicTrack[index].id == -2) {
                    return false
                }
            }
        },
        computed: {
            currentMusicInfo() {
                return this.musicTrack[this.musicTrackIndex];
            }
        },
        beforeUnmount() {
            this.audioManager.destroyThisManager()
        },
        created() {
            // 检测 Tauri API 是否存在
            if (window.tauri) {
                this.appState.isTauri = true
                console.log('应用是通过 Tauri 启动的。');

                // 你可以进一步使用 Tauri 提供的 API
                // 例如，获取Tauri版本
                window.tauri.getVersion().then(version => {
                    console.log(`Tauri 版本: ${version}`);
                }).catch(error => {
                    console.error('无法获取 Tauri 版本:', error);
                });
            }
        },
        mounted() {
            this.audioManagerConstruct(this.currentMusicInfo.src)
        },
        watch: {}
    }
</script>

<template>
    <topBar :titleOffsetTop="titleOffsetTop" :leftBarState="leftBarState">
        <template #title>
            <textspawn :text="this.title" />
        </template>
    </topBar>

    <div class="bottom">
        <leftBar @leftBarChange="(newState)=>{leftBarState = newState}">
            <template #buttons>
                <!--音乐库-->
                <iconWithText style="width: 100%;" @click="this.$router.push('/')"
                    :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-house-fill"></i>
                    </template>
                    <template #text>
                        音乐库
                    </template>
                </iconWithText>

                <!--音乐目录-->
                <iconWithText style="width: 100%;" @click="this.$router.push('/musicFolder/')"
                    :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-folder-fill"></i>
                    </template>
                    <template #text>
                        音乐来源
                    </template>
                </iconWithText>

                <iconWithText style="width: 100%;" @click="this.$router.push('/setting/')"
                    :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-gear-fill"></i>
                    </template>
                    <template #text>
                        设置
                    </template>
                </iconWithText>
            </template>
        </leftBar>
        <rightBlock :leftBarState="leftBarState"></rightBlock>

    </div>

    <musicInfoPage></musicInfoPage>
</template>

<style scoped>
    .bottom {
        display: flex;
        height: calc(100vh - 48px);
        width: fit-content;
        z-index: 1;
        margin-left: -8px;
    }
</style>