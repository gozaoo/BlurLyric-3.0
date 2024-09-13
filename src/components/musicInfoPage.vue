<script>
    import lazyLoadCoverImage from './base/lazyLoadCoverImage.vue';
    import buttom_icon_circleBackground from './base/buttom_icon_circleBackground.vue';
    import playModeSvg from './musicInfoPageComponents/playModeSvg.vue';
    import drag from '../js/drag';
    import textSpawn from './base/text-spawn.vue';
    import anime from 'animejs';

    import Background from './background.vue';
    export default {
        data() {
            return {
                style: {
                    musicDetailRender: {
                        transformX: 0,
                    }
                },
                eventListenerRemovers: [],
                dragInfo: null,
                musicInfoPagePosition: 'bottom'
            }
        },
        components: {
            lazyLoadCoverImage,
            buttom_icon_circleBackground,
            playModeSvg,
            textSpawn,
            Background
        },
        inject: ['currentMusicInfo', 'audioState', 'audioManager', 'changePlayMode', 'trackState', 'musicTrack',
            'nextMusic', 'prevMusic', 'getNextMusicIndex', 'getPrevMusicIndex'
        ],
        mounted() {
            this.onBottomListener()
        },
        beforeUnmount() {
            this.eventListenerRemovers.map((value) => {
                value()
            })
        },
        beforeUnmount() {
            this.eventListenerRemovers.map((value) => value())
        },
        methods: {
            toTop(info) {
                console.log(-info.speedY);
                this.eventListenerRemovers.map((value) => value())
                this.onTopListener()
                anime({
                    targets: this.$refs.musicInfoPageRow,
                    easing: 'spring(1, 80, 14,' + (-info.speedY).toFixed(2) + ')',
                    translateY: -document.body.offsetHeight,

                    complete: (anim) => {

                    }
                })

                anime({
                    targets: this.$refs.barDetail,
                    easing: 'linear',
                    opacity: 0,
                    duration: 100
                })
            },
            toBottom(info) {
                // this.eventListenerRemovers.push(callBack_drag.destroy)
                anime.set(this.style.musicDetailRender,{
                    transformX: 0,
                })

                this.eventListenerRemovers.map((value) => value())
                anime({
                    targets: this.$refs.musicInfoPageRow,
                    easing: 'spring(1, 80, 14,' + (info.speedY).toFixed(2) + ')',
                    translateY: -88,
                })
                anime({
                    targets: this.$refs.barDetail,
                    easing: 'linear',
                    opacity: 1,
                    duration: 100
                })

                this.onBottomListener()
            },
            onBottomListener() {
                let musicControlBar_animeJsCallBack = null
                let lastTransformX, lastTransformY
                let callBack_drag = drag.create(this.$refs.musicControlBar,
                    (info) => {
                        lastTransformX = this.style.musicDetailRender.transformX
                        if (musicControlBar_animeJsCallBack != null) musicControlBar_animeJsCallBack.pause()

                        if (info.offsetDirectionX != 'none') {
                            this.style.musicDetailRender.transformX = info.offsetX + lastTransformX
                        }
                        anime.set(this.$refs.musicInfoPageRow, {
                            backdropFilter: 'blur(30px)',
                        })
                        anime({
                            targets: this.$refs.musicInfoPageRow,
                            background: 'rgba(0,0,0,0.1)',
                            easings: 'linear',
                            // duration: 100
                        })
                        this.dragInfo = info
                    },
                    (info) => {

                        if (info.offsetDirectionX != 'none') {
                            this.style.musicDetailRender.transformX = info.offsetX + lastTransformX
                        }

                        if (info.offsetY < -100) {
                            if (navigator.vibrate) navigator.vibrate(50);
                            this.toTop(info)


                        }
                        this.dragInfo = info

                    },
                    (info) => {

                        if (info.offsetX < -100 || info.speedX < -1) {
                            if (navigator.vibrate) navigator.vibrate(50);
                            this.nextMusic()
                        }
                        if (info.offsetX > 100 || info.speedX > 1) {
                            if (navigator.vibrate) navigator.vibrate(50);
                            this.prevMusic()
                        }
                        musicControlBar_animeJsCallBack = anime({
                            targets: this.style.musicDetailRender,
                            transformX: 0,
                            easing: 'spring(1, 80, 14,0)'
                        })

                        anime({
                            targets: this.$refs.musicInfoPageRow,
                            background: 'rgba(0,0,0,0.0625)',
                            easings: 'linear',
                            // duration: 100
                        })
                        this.dragInfo = null

                    })

                this.eventListenerRemovers.push(callBack_drag.destroy)
            },
            onTopListener() {

                let callBack_drag = drag.create(this.$refs.cover,
                    (info) => {
                        this.dragInfo = info
                        console.log('a');

                    },
                    (info) => {
                        if (info.offsetY > 100) {
                            if (navigator.vibrate) navigator.vibrate(50);
                            this.toBottom(info)
                        }
                        this.dragInfo = info
                        console.log('a');

                    },
                    (info) => {

                    })

                this.eventListenerRemovers.push(callBack_drag.destroy)
            }
        }
    }
</script>
<template>
    <div ref="musicInfoPageRow" style="
        transform: translateY(-88px);background:rgba(0,0,0,0.0625);
        " class="global_backgroundblur_light musicInfoPageRow">

        <div class="relativeBox">
            <div ref="cover" class="cover">
                <lazyLoadCoverImage class="blur" :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                <lazyLoadCoverImage :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
            </div>

            <!-- <div class="cover">
                    <lazyLoadCoverImage class="blur" :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                    <lazyLoadCoverImage :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                </div> -->
            <div ref="musicControlBar" class="musicControlBar">
                <div class="cover placeholder">
                </div>
                <div ref="barDetail" class="detail">
                    <div :style="{
                        transform:'translateX('+style.musicDetailRender.transformX+'px)'
                    }" class="dragRender">
                        <div class="prev">
                            <div class="event">上一首</div>
                            <div class="name">{{ musicTrack[getNextMusicIndex()].name}}</div>
                        </div>
                        <div class="currentMusic">
                            <div class="name">
                                <textSpawn :text="currentMusicInfo.name" />

                            </div>
                            <div class="artists">
                                <span v-for="(value,index) in currentMusicInfo.ar">
                                    <textSpawn :text="value.name + ((index!=currentMusicInfo.ar.length-1)?'/':'')" />
                                </span>
                                <span v-if="currentMusicInfo.al.id != -2"> - </span>{{  currentMusicInfo.al.name }}

                            </div>
                        </div>
                        <div class="next">
                            <div class="event">下一首</div>
                            <div v-if="trackState.playMode != 'randomPlay'" class="name">
                                {{ musicTrack[getNextMusicIndex()].name}}</div>
                            <div v-if="trackState.playMode == 'randomPlay'" class="name">随机播放</div>
                        </div>
                    </div>
                </div>
                <div v-show="currentMusicInfo.id != -2" class="control">
                    <buttom_icon_circleBackground @click="prevMusic()">
                        <template #icon>
                            <i class="bi bi-skip-start-fill"></i>
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground
                        @click="(audioState.playing == true)?audioManager.pause():audioManager.play()"
                        class="playButtom">
                        <template #icon>
                            <div style="transform: scale(1.5);transform-origin: 50% 50%;">
                                <i v-if="audioState.playing == true" class="bi bi-pause-fill"></i>
                                <i v-if="audioState.playing == false" class="bi bi-play-fill"></i>
                            </div>
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground @click="nextMusic()">
                        <template #icon>
                            <i class="bi bi-skip-end-fill"></i>
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground @click="changePlayMode">
                        <template #icon>
                            <playModeSvg style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;">
                            </playModeSvg>
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground>
                        <template #icon>
                            <i style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;"
                                class="bi-volume-up bi"></i>
                            <!-- <playModeSvg  style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;"></playModeSvg> -->
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground>
                        <template #icon>
                            <i style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;"
                                class="bi-three-dots bi"></i>
                        </template>
                    </buttom_icon_circleBackground>
                </div>
            </div>
            <div class="mainContainer">
             <!-- <background /> -->
                <div class="controlBar">
                    <div class="tapBar"></div>
                </div>

            </div>
        </div>
    </div>
</template>
<style scoped>
    .tapBar{
        margin: 0.6em auto;
        height: .8em;
        cursor: n-resize;
        width: 5em;
        border-radius: 1em;
        background-color: #0002
    }
    .controlBar{
        height: 2em
    }
    .mainContainer{
        font-size: 1rem;
        z-index: 0;
    }
    .musicInfoPageRow {
        position: absolute;
        height: 100%;
        width: 100%;
        left: 0;
        top: 100%;
        background: rgba(0, 0, 0, 0.0625);
        -webkit-user-drag: none !important;
        user-select: none;
        z-index: 100;
        box-sizing: border-box;
        border: 1px solid #0001;
        box-shadow: 0 0px 15px rgba(0, 0, 0, 0.14);
        /* transform: translateY() */
    }

    .relativeBox {
        display: relative;
        height: 100%;
        width: 100%;
    }

    .musicControlBar {
        position: absolute;
        height: 88px;
        width: 100%;
        top: 0;
        box-sizing: border-box;
        display: flex;
        gap: 17px;
        padding: 17px;
        overflow: hidden;
        z-index: 0;
    }

    .cover {
        position: absolute;
        height: 54px;
        left: 17px;
        top: 17px;
        z-index: 1;
        aspect-ratio: 1/1;
    }

    .cover.placeholder {
        min-height: 54px !important;
        width: 54px !important;
        user-select: none;
        position: relative;
        flex: 0 1 0;
    }

    .cover>* {
        border-radius: 1%;
        overflow: hidden;
        height: inherit;
        width: inherit;
    }

    .cover>.blur {
        filter: blur(12px);
        position: absolute;
        transform-origin: 50% 100%;
        transform: scale(0.85);
    }

    .detail {
        display: flex;
        flex-direction: column;
        justify-content: center;
        height: 100%;
        position: relative;
        width: 100%;
        flex: 1 0 1;
        margin: 0 -10px;
        white-space: nowrap;
        padding: 0 10px;
        user-select: none;
        mask-image: linear-gradient(90deg, #0000 0%, #000f 10px, #000f calc(100% - 10px), #0000 100%)
    }

    .detail>.dragRender {
        position: absolute;
        display: flex;
        width: inherit;
    }

    .prev {
        position: absolute;
        text-align: end;
        transform: translateX(calc(-100% - 10px))
    }

    .next {
        transform: translateX(-10px);
        position: absolute;
        width: max-content;
        left: 100%;
    }

    .prev .event,
    .next .event {
        font-weight: 900;
        font-size: 18px;
        color: #0066cd;
    }

    .currentMusic>.name {
        font-size: 18px;
        font-weight: 900;
        color: #333;
    }

    .currentMusic>.artists,
    .prev>.name,
    .next>.name {
        font-size: 14px;
        /* font-weight: 900; */
        color: #676767;

    }

    .control {
        display: flex;
        color: #333;
        align-items: center;
        gap: 6px;
        flex: 0 1 0;
        margin-left: auto;
        font-size: 25px;
    }

    .control>* {
        flex-shrink: 0;
        height: 30px;
    }

    .playButtom {
        font-size: 28px;
        height: 30px;
        padding: 0.4em;
    }

    @media screen and (max-width: 560px) {
        .control>* {
            display: none;
        }

        .control>*.playButtom {
            display: inline-flex;
        }
    }
</style>