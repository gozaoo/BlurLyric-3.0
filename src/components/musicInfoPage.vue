<script>
    import "bootstrap-icons/font/bootstrap-icons.css";
    import background from "./background.vue";
    import elemListener from "../js/elemListener.js"
    import anime from 'animejs/lib/anime.es.js';
    import lyricVue from "./lyric.vue";
    import landscapeVue from "./musicInfoPageComponents/landscape.vue";
    import portraitVue from './musicInfoPageComponents/portrait.vue'
    export default {
        data() {
            return {
                displayState: 'buttom',
                oriDisplayState: 'buttom',
                eventTempCache: (args) => {
                    switch (args.direction) {
                        case "up":
                        case "down":
                            this.changeDisplayState(args.direction, Number((args.speed * 0.15).toFixed(0)))
                            break;
                        case "left":
                        case "right":
                            break;
                        default:
                            break;
                    }
                },
                slideEventReturn: {
                    removeSlideEvent: () => {}
                },
                windowsResizeReturn: {
                    removeWindowsResize: () => {}
                },
                tempSetTimeoutSaver: setTimeout(() => {}, 0),
                screenMode: 'landscape',
                childRefs: {}
            }
        },
        components: {
            background,
            lyricVue,
            landscapeVue,
            portraitVue

        },
        methods: {
            changeDisplayState(area, speed) {
                speed = Math.abs(speed).toFixed(0);
                if (speed > 15) speed = 15;
                if (speed === NaN) speed = 5
                const self = this
                switch (area) {
                    case 'up':
                        if (this.displayState == 'top') return
                        this.displayState = 'top'
                        this.oriDisplayState = 'top'
                        anime({
                            targets: this.$refs.player,
                            height: [this.$refs.player.offsetHeight + "px", window.innerHeight + "px"],
                            backgroundColor: "#ddddddff",
                            easing: 'spring(1,100, 60, ' + speed + ')',
                        })
                        anime({
                            targets: this.childRefs.menu,
                            easing: 'linear',
                            duration: 100,
                            opacity: 1,
                        })
                        anime({
                            targets: this.$refs.miniControlBar,
                            opacity: 0,
                            right: ['0px', '-' + this.$refs.miniControlBar.offsetWidth + 'px'],
                            easing: 'spring(1,100, 60, ' + speed + ')',
                        })
                        anime({
                            easing: 'spring(1, 100, 60,  ' + speed + ')',
                            targets: this.$refs.coverImage,
                            left: this.childRefs.coverImagePlaceHolder.offsetLeft,
                            top: this.childRefs.coverImagePlaceHolder.offsetTop,
                            height: this.childRefs.coverImagePlaceHolder.offsetWidth + 'px',
                        })
                        this.slideEventReturn.removeSlideEvent()
                        this.slideEventReturn = elemListener.addSlideEvent(this.$refs.coverImage, this.eventTempCache, {
                            threshold: 100
                        })
                        this.tempSetTimeoutSaver = setTimeout(() => {
                            this.$nextTick(() => {
                                this.$refs.player.style.height = '100%'
                                this.$refs.highQualityView.style.display = 'block'
                            })
                        }, 1300);
                        setTimeout(() => {
                            this.$nextTick(() => {
                                this.$refs.highQualityView.style.display = 'block'
                            })
                        }, 400);
                        this.windowsResizeReturn = elemListener.onWindowsResize(() => {

                        anime({
                            easing: 'spring(1, 100, 60,  10)',
                            targets: this.$refs.coverImage,
                            left: this.childRefs.coverImagePlaceHolder.offsetLeft,
                            top: this.childRefs.coverImagePlaceHolder.offsetTop,
                            height: this.childRefs.coverImagePlaceHolder.offsetWidth + 'px',
                        })
                        })
                        break;

                    case 'down':
                        clearTimeout(this.tempSetTimeoutSaver)
                        if (this.displayState == 'buttom') return
                        this.displayState = 'buttom'
                        this.oriDisplayState = 'buttom'

                        this.$refs.highQualityView.style.display = 'none'

                        anime({
                            targets: this.$refs.player,
                            height: [this.$refs.player.offsetHeight + "px", '100px'],
                            easing: 'spring(1, 100, 60,' + speed + ')',
                            finished: () => {
                                anime({
                                    targets: self.$refs.player,
                                    backgroundColor: "#dddddd55",
                                    easing: 'spring(1, 100, 60,  ' + speed + ')',
                                })
                            }
                        })
                        anime({
                            targets: this.childRefs.menu,
                            easing: 'linear',
                            duration: 100,
                            opacity: 0,
                        })
                        anime({
                            targets: this.$refs.miniControlBar,
                            opacity: 1,
                            right: '0px',
                            duration: 300,
                            easing: 'spring(1, 100, 60,  ' + speed + ')',
                        })

                        anime({
                            targets: this.$refs.coverImage,
                            left: 0,
                            easing: 'spring(1, 100, 60,  ' + speed + ')',
                            top: 0,
                            height: '58px',
                        })

                        this.slideEventReturn.removeSlideEvent()
                        this.slideEventReturn = elemListener.addSlideEvent(this.$refs.player, this.eventTempCache, {
                            threshold: 100
                        })
                        this.windowsResizeReturn.removeWindowsResize()
                        break;

                    default:
                        break;
                }
            },

            formTime(sec) { //秒数转化为mm:ss
                let s = sec % 60 < 10 ? ('0' + sec % 60) : sec % 60
                let min = Math.floor(sec / 60) < 10 ? ('0' + Math.floor(sec / 60)) : Math.floor(sec / 60)
                return min + ':' + s
            },
            bindChildRefs(info) {
                console.log(info);
                this.childRefs[info.path] = info.element
            }
        },
        props: {
            musicInfo: Object,
            controls: Object,
            playerState: Object,
            lyric: Object,
            audio: HTMLAudioElement
        },
        watch: {
            musicInfo: {
                handler: async function (newVal) {},
                deep: true
            }
        },
        created() {
            const self = this
            this.$nextTick(() => {
                elemListener.addOnhoverListener(this.$refs.player, (state) => {
                    if (this.displayState == "top") return;
                    this.displayState = (state) ? 'wait' : this.oriDisplayState
                })
                this.slideEventReturn = elemListener.addSlideEvent(this.$refs.player, this.eventTempCache, {
                    threshold: 100
                })
            })
        }
    }
</script>

<template>
    <div ref="player" :class="['player',displayState]">
        <div id="audioProgress" v-if="displayState!='top'" class="processbar">
            <div :style="{'width': (playerState.icurrentTime / playerState.durationTime * 100).toFixed(2) + '%'}"
                class="progress"></div>
            <!-- <span>{{ playerState.currentTime }}</span> -->
        </div>
        <div class="relativeRow">
            <background :dynamic="false" :imgSrc="musicInfo.al.picUrl" :mainDisplay="displayState" />

            <div ref="coverImage" class="alPicTure">
                <div class="relativeRow">

                    <img class="blur" :src="musicInfo.al.picUrl + '?param=128y128'" alt="" srcset="">
                    <img ref="highQualityView" class="highQualityView" :src="musicInfo.al.picUrl + '?param=1024y1024'"
                        alt="">
                    <img class="view" :src="musicInfo.al.picUrl + '?param=128y128'" alt="" srcset="">

                </div>
            </div>
            <div ref="miniControlBar" class="hiddenControlBar">
                <div class="flexRow">
                    <div class="info">
                        <h1 class="title">{{ musicInfo.name }}</h1>
                        <h2 class="artist"><span v-for="(item,e,index) in musicInfo.ar">{{ item.name + ' ' }}</span>
                        </h2>
                    </div>
                    <div class="bottoms">
                        <i @click="controls.prevTrack()" class="bi bi-skip-start-fill"></i>
                        <i @click="controls.play()"
                            :class="['bi',(playerState.audioState)?'bi-pause-fill':'bi-play-fill','notice']"></i>
                        <i @click="controls.nextTrack()" class="bi bi-skip-end-fill"></i>
                    </div>
                </div>
            </div>
            <landscapeVue
            @bindRef="bindChildRefs"
            :playerState="
    playerState
  " :controls="
    controls
  " :audio="audio" :lyric="lyric" :musicInfo="musicInfo"></landscapeVue>
        </div>

    </div>
</template>
<style scoped>
    .player {
        position: absolute;
        user-select: none;
        -webkit-user-drag: none;
        bottom: 0;
        left: 0px;
        width: 100vw;
        border-top: #ddd9 solid 1px;
        height: 100px;
        box-sizing: border-box;
        padding: var(--adaptiveSize);
        overflow: hidden;
        background-color: #ddd5;
        backdrop-filter: blur(6vmin) saturate(200%);
        --adaptiveSize: min(2.5vh, 2vw);
        z-index: 9999;
        box-shadow: 0 0 5vmin #00000005;
        transition: background-color .3s, backdrop-filter 0s 0s,var(--adaptiveSize) 0s;
    }

    .player:hover {
        background-color: #ddd;
    }

    .player.top,
    .player.wait {
        backdrop-filter: blur(0) saturate(0);
        background-color: #ddd;
        transition: background-color .3s, backdrop-filter 0s 0.3s;
    }

    .processbar {
        position: absolute;
        top: 0px;
        left: 0;
        height: 5px;
        /* background: #ddd; */
        /* box-shadow: rgba(0, 0, 0, 0.4) 0px 16px 32px; */
        width: 100%;
    }

    .processbar>.progress {
        height: 5px;
        /* transition: width 1s linear; */
        background: #0005;
    }

    .player>.relativeRow {
        width: 100%;
        height: 100%;
    }

    .relativeRow {
        position: relative;
    }

    .alPicTure {
        height: 58px;
        aspect-ratio: 1 / 1;
        left: 0px;
        position: absolute;
    }

    .alPicTure>.relativeRow {
        aspect-ratio: 1 / 1;
        height: inherit;
    }

    .alPicTure img {
        /* border-radius: 1%; */
        border-radius: 1%;
        height: inherit;
        z-index: -1;
        -webkit-user-drag: none;
    }

    .alPicTure img.view {
        border: #0001 solid 1px;

    }

    .alPicTure img.blur {
        position: absolute;
        z-index: -1;

        filter: blur(1.5vmin) saturate(200%)
    }

    .alPicTure img.highQualityView {
        position: absolute;
        z-index: 2;
        /* filter: blur(2vmin) */
        border: #0001 solid 1px;
        display: none;
        /* transition: display 0 0s; */

    }

    /*
     img.highQualityView{
        display: block;
        transition: display 0 0.3s;
    } */

    .hiddenControlBar {
        display: block;
        position: absolute;
        right: 0px;
        /* top: 20px; */
        height: 60px;
        width: calc(100% - 60px);
        z-index: 2;
    }

    .hiddenControlBar .info {
        display: flex;
        align-items: flex-start;
        flex-direction: column;
        justify-content: center;
        height: 60px;
    }

    .flexRow {
        display: flex;
        padding-left: 16px;
    }

    .hiddenControlBar h1.title {
        margin: 0;
        font-size: 19px;
    }

    .hiddenControlBar h2.artist {
        margin: 0;
        font-size: 16px;
        color: #000b;

    }

    .hiddenControlBar .bottoms {
        position: relative;
        display: flex;
        align-items: center;
        margin-left: auto;
        font-size: 28px;
        margin-right: 20px;
        justify-content: center;
        color: #000d;
        height: 60px;
        gap: 20px;
    }

    .hiddenControlBar .bottoms .notice {
        font-size: 40px;
    }

    .hiddenControlBar .bottoms>* {
        position: relative;

    }

    div.musicControls>button:hover::after,
    .hiddenControlBar .bottoms>*:hover::after {
        position: absolute;
        background-color: #0001;
        width: 100%;
        border-radius: 50%;
        /* height: 100%; */
        aspect-ratio: 1/1;
        left: 0;
        top: 50%;
        transform: translate(0, -50%) scale(1.3);
        content: "";
    }
</style>