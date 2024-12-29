<script>
import topBar from './components/topBar.vue'
import rightBlock from './components/rightBlock.vue'
import leftBar from './components/leftBar.vue'
import musicInfoPage from './components/musicInfoPage.vue'
import manager from './api/manager'
import messageDisplay from './components/messageDisplay.vue'
import {
    computed,
    ref,
    onMounted
} from 'vue'
import baseMethods from './js/baseMethods'
import anime from 'animejs/lib/anime.es'

let templateEmptyMusicTrack = [{
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
    },
    src: null
}]

export default {
    name: 'app',
    components: {
        topBar,
        leftBar,
        rightBlock,
        musicInfoPage,
        messageDisplay
    },
    data() {
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
                local: [],
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
    provide() {
        return {
            scrollState: computed(() => this.scrollState),
            leftBarState: computed(() => this.leftBarState),
            config: computed(() => this.config),
            // runOnTauri: (window.__TAURI_INTERNALS__)? true: false,
            currentMusicInfo: computed(() => this.musicTrack[this.musicTrackIndex]),
            pushMusic: this.pushMusic,
            pushMusicTrack: this.pushMusicTrack,
            coverMusicTrack: this.coverMusicTrack,
            cleanUpMusicTrack: this.cleanUpMusicTrack,
            checkMusicListIsEmpty: this.checkMusicListIsEmpty,
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
            source: computed(() => this.source),
            regResizeHandle: this.regResizeHandle,

            messageList: computed(() => this.messageList),
            regMessage: this.regMessage,
            getAllMessages: this.getAllMessages,
            destoryMessage: this.destoryMessage,

        };
    },
    methods: {
        checkMusicListIsEmpty() {
            if (this.musicTrack[0].id == -2) {
                return true
            };
            return false
        },
        cleanUpMusicTrack() {
            this.pushMusicTrack(templateEmptyMusicTrack)
        },
        async pushMusic(singleSong) {


            if (this.checkMusicListIsEmpty()) {
                this.musicTrack.length = 0
                this.musicTrack.push(singleSong)

                await this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex])
                this.audioManager.play()
                this.regMessage({
                    type: 'Message',
                    content: '已添加音乐《' + singleSong.name + '》至列表末 '
                })
                return;
            };

            this.musicTrack.push(singleSong)

            this.regMessage({
                type: 'Message',
                content: '已添加音乐《' + singleSong.name + '》至列表末 '
            })
        },
        async pushMusicTrack(musicTrack) {
            if (this.checkMusicListIsEmpty()) {
                this.musicTrack.length = 0;
                this.musicTrack.concat(musicTrack);
                await this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex])
                this.audioManager.play()
                return;
            }
            this.musicTrack.concat(musicTrack)
        },
        async coverMusicTrack(musicTrack) {

            if (this.audioManager) this.audioManager.destroyThisManager()

            this.musicTrack = musicTrack;
            this.musicTrackIndex = 0;

            await this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex])
            this.audioManager.play()
        },
        regResizeHandle(key, event) {
            this.resizeEvent[key] = event;
            return {
                cancelReg: () => {
                    // this.resizeEvent[key] = undefined
                    delete this.resizeEvent[key]
                }
            }
        },
        handleResize() {
            for (const key in this.resizeEvent) {
                if (Object.prototype.hasOwnProperty.call(this.resizeEvent, key)) {
                    const element = this.resizeEvent[key];
                    element()
                }
            }

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

        audioManagerConstruct(newSong) {
            let timeStamps = Date.now()

            let newAudio = document.createElement('audio');

            newAudio.volume = this.audioState.volume * 1
            // const isFilePath = ;
            let stillAvalible = true
            const checkConstructAvalible = () => {
                if (stillAvalible == true) {
                    return true;
                } else {
                    destroyThisManager()
                    return false
                }
            }
            const destroyObjectURLMethod = () => {

            }
            if (baseMethods.isPossibleLocalPath(newSong.src)) {
                // 如果是文件路径，使用manager.tauri.getMusicFile获取Base64信息
                // 假设binaryData已经是Uint8Array类型
                manager.tauri.getMusicFile(newSong.id).then(result => {
                    if (checkConstructAvalible()) { newAudio.src = result.objectURL; destroyObjectURLMethod = result.destroyObjectURL }

                }
                ).catch(error => {
                    console.error('Error fetching music file:', error);
                });
            } else {
                // 如果不是文件路径，直接设置src
                newAudio.src = newSong.src;
                // checkAndPlay();
            }

            this.setupMediaSession();
            const {
                audioState
            } = this;

            let updateAudioState = (state, value) => {
                if (stillAvalible == true)
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
                if (checkIsCurrentConstruct()) {
                    updateAudioState('playing', true);
                    currentEventHandler();
                }
            };

            let pauseHandler = () => {
                if (checkIsCurrentConstruct()) {
                    updateAudioState('playing', false);
                }
            };

            let timeupdateHandler = () => {
                // if(audio)
                if (checkIsCurrentConstruct() && checkConstructAvalible()) {
                    updateAudioState('currentTime', newAudio.currentTime);
                }
                // 开始处理播放结束事件
                if (this.trackState.playMode == 'stopAfterSingle') return;
                let leastTime = newAudio.duration - newAudio.currentTime
                if (this.config.audio.smartStreamAudioList == true && leastTime < this.config.audio.audioStreamDuration) {
                    this.transitionNextMusic()
                } else if (leastTime == 0) {
                    let nextMusicIndex = this.getNextMusicIndex();
                    this.musicTrackIndex = nextMusicIndex;
                    const nextSong = this.musicTrack[nextMusicIndex];
                    destroyThisManager()

                    this.audioManagerConstruct(nextSong);

                    this.audioManager.play()
                }
            };

            let currentEventHandler = () => {
                if (checkIsCurrentConstruct() && checkConstructAvalible() && audioState.playing == true && newAudio.duration != NaN) {
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
                    if (checkConstructAvalible()) newAudio.play();

                } else {
                    newAudio.addEventListener('canplay', () => {

                        try {
                            if (checkConstructAvalible()) newAudio.play();
                        } catch {

                        }
                    });
                }
            };

            const checkIsCurrentConstruct = () => {
                if (this.audioManager.timeStamps == timeStamps) {
                    return true;
                } else {
                    return false
                }
            }
            newAudio.addEventListener('loadeddata', loadeddataHandler);
            newAudio.addEventListener('playing', playingHandler);
            newAudio.addEventListener('pause', pauseHandler);
            newAudio.addEventListener('timeupdate', timeupdateHandler);


            const cancelListener = () => {
                newAudio.removeEventListener('loadeddata', loadeddataHandler);
                newAudio.removeEventListener('playing', playingHandler);
                newAudio.removeEventListener('pause', pauseHandler);
                newAudio.removeEventListener('timeupdate', timeupdateHandler);
            };

            let destroyThisManager = () => {
                console.log('des');
                stillAvalible = false
                newAudio.pause();

                // 如果是文件路径，销毁ObjectURL
                baseMethods.isPossibleLocalPath(newSong.src) ? destroyObjectURLMethod() : '';

                // 重置音频状态
                updateAudioState('playing', false);

                // 移除监听器
                newAudio.src = undefined
                newAudio.remove();
                cancelListener();
            };
            // 创建音频管理器
            let audioManager = {
                audioDom: newAudio,
                cancelListener,
                destroyThisManager: () => {
                    destroyThisManager()
                },
                play: checkAndPlay,
                pause: () => {
                    newAudio.pause()
                },
                timeStamps
            }
            this.audioManager = audioManager;
            return audioManager;
        },

        setupMediaSession() {
            if ("mediaSession" in navigator) {
                const metadata = new MediaMetadata({
                    title: this.currentMusicInfo.name, // 音频标题
                    artist: this.currentMusicInfo.ar.map(artist => artist.name).join('/'), // 艺术家
                    album: this.currentMusicInfo.al.name, // 专辑名称
                    // artwork: [{
                    //     src: this.currentMusicInfo.al.picUrl
                    // }] // 专辑封面图片
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
            let modeName = {
                'loopPlaylist': "列表循环", 'loopSingle': '单曲循环', 'stopAfterSingle': '播完本曲暂停', 'randomPlay': '随机播放', 'smartRecommend': '智能推荐（暂时不可用）'
            }

            this.regMessage({
                type: 'Message',
                content: '播放模式已经调为 ' + modeName[this.trackState.playMode]
            })

        },
        getNextMusicIndex() {
            switch (this.trackState.playMode) {
                case 'loopPlaylist': {
                    let nextIndex = this.musicTrackIndex + 1;
                    if (nextIndex >= this.musicTrack.length) nextIndex = 0;
                    return nextIndex;
                }

                case 'loopSingle': {
                    return this.musicTrackIndex; // 重复播放当前歌曲
                }
                case 'stopAfterSingle': {
                    let nextIndex = this.musicTrackIndex + 1;
                    if (nextIndex >= this.musicTrack.length) nextIndex = 0;
                    return nextIndex;
                }

                case 'randomPlay': {
                    let randomIndex = this.musicTrackIndex;
                    if (this.musicTrack.length > 1) {
                        while (randomIndex === this.musicTrackIndex) {
                            randomIndex = Math.floor(Math.random() * this.musicTrack.length);
                        }
                    }

                    return randomIndex;
                }

                case 'smartRecommend': {
                    // 暂时按照下一首处理
                    let nextIndexForSmart = this.musicTrackIndex + 1;
                    if (nextIndexForSmart >= this.musicTrack.length) nextIndexForSmart = 0;
                    return nextIndexForSmart;
                }

                default: {
                    return 0;
                }
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
        async nextMusic() {
            this.musicTrackIndex = this.getNextMusicIndex();

            if (this.checkMusicIsUsable(this.musicTrackIndex) == false) {
                return
            }

            if (this.audioManager) this.audioManager.destroyThisManager()

            await this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex])
            this.audioManager.play()
        },
        async prevMusic() {
            this.musicTrackIndex = this.getPrevMusicIndex();
            if (this.checkMusicIsUsable(this.musicTrackIndex) == false) {
                return
            }
            if (this.audioManager) this.audioManager.destroyThisManager()

            await this.audioManagerConstruct(this.musicTrack[this.musicTrackIndex])
            this.audioManager.play()
        },
        checkMusicIsUsable(index) {
            if (this.musicTrack[index].id == -2) {
                return false
            }
        },
        async transitionNextMusic() {
            if (this.transitionNextMusicWorking == true) return;
            this.transitionNextMusicWorking = true;

            console.log("working");

            // debugger
            let nextMusicIndex = this.getNextMusicIndex();
            this.musicTrackIndex = nextMusicIndex;
            const nextSong = this.musicTrack[nextMusicIndex];

            let oldAudioManager = this.audioManager
            let time = 1000 * (oldAudioManager.audioDom.duration - oldAudioManager.audioDom.currentTime); // 播放剩余时间

            // 切换到新音频
            this.audioManagerConstruct(nextSong)
            const newAudio = this.audioManager
            newAudio.audioDom.volume = 0.5 * this.audioState.volume;

            // 创建音频管理器
            this.audioManager.play();

            // 处理音频切换时的过渡效果
            anime({
                targets: newAudio.audioDom,
                duration: time,
                volume: 1 * this.audioState.volume,
                easing: "linear"
            });

            anime({
                targets: oldAudioManager.audioDom,
                duration: time,
                volume: 0,
                easing: 'linear',
                complete: () => {
                    oldAudioManager.destroyThisManager()
                    this.transitionNextMusicWorking = false;

                }
            });

            // 更新播放状态
        },
        async getAllMessages() {
            return this.messageList;
        },
        async regMessage(message) {
            let timeStamp = Date.now()
            message["timeStamp"] = timeStamp
            /**
             * 对于 message
             * {
             *      type: String // Message或Alert
             *      timeStamp: Number // 作为时间标识符
             *      content: String // 作为内容
             *      leastTime: Number // Message或Alert的持续时间，选填
             * }
             */
            this.messageList.push(message);
            setTimeout(() => {
                this.messageList = this.messageList.filter(_message => _message.timeStamp != timeStamp);
            }, message.leastTime || 7 * 1000);
            return this.messageList.length - 1
        },
        // async destoryMessage(index) {
        //     this.messageList.splice(index, 1);
        // }
    },
    computed: {
        currentMusicInfo() {

            return this.musicTrack[this.musicTrackIndex];
        }
    },
    beforeUnmount() {
        if (this.audioManager) this.audioManager.destroyThisManager()
    },
    created() {
        // 检测 Tauri API 是否存在

        this.appState.runOnTauri = (window.__TAURI_INTERNALS__) ? true : false;

        window.addEventListener('resize', () => {
            this.handleResize()
        })
    },
    mounted() {
        // this.audioManagerConstruct(this.currentMusicInfo)
    },
    watch: {}
}
</script>

<template>
    <messageDisplay />
    <topBar :titleOffsetTop="titleOffsetTop" :leftBarState="leftBarState">
        <template #title>
            <textspawn :text="this.title" />
        </template>
    </topBar>

    <div class="bottom">
        <leftBar @leftBarChange="(newState) => { leftBarState = newState }">
            <template #buttons>
                <!--音乐库-->
                <iconWithText style="width: 100%;" @click="this.$router.push('/')"
                    :type="(leftBarState == 'short') ? 'hidden' : null">
                    <template #svg>
                        <i class="bi bi-house-fill"></i>
                    </template>
                    <template #text>
                        音乐库
                    </template>
                </iconWithText>

                <!--音乐目录-->
                <iconWithText style="width: 100%;" @click="this.$router.push('/musicFolder/')"
                    :type="(leftBarState == 'short') ? 'hidden' : null">
                    <template #svg>
                        <i class="bi bi-folder-fill"></i>
                    </template>
                    <template #text>
                        音乐来源
                    </template>
                </iconWithText>


                <iconWithText style="width: 100%;" @click="this.$router.push('/musicTrack/')"
                    :type="(leftBarState == 'short') ? 'hidden' : null">
                    <template #svg>
                        <i class="bi bi-music-note-list"></i>
                    </template>
                    <template #text>
                        播放列表
                    </template>
                </iconWithText>
                <iconWithText style="width: 100%;" @click="this.$router.push('/setting/')"
                    :type="(leftBarState == 'short') ? 'hidden' : null">
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