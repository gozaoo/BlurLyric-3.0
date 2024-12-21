<script>
import lazyLoadCoverImage from './base/lazyLoadCoverImage.vue';
import buttom_icon_circleBackground from './base/buttom_icon_circleBackground.vue';
import playModeSvg from './musicInfoPageComponents/playModeSvg.vue';
import drag from '../js/drag';
import textSpawn from './base/text-spawn.vue';
import anime from 'animejs';
import background from './musicInfoPageComponents/background.vue';
import contain from './musicInfoPageComponents/contain.vue';
import {
    computed,
} from 'vue'
import baseMethods from '../js/baseMethods';
export default {
    data() {
        return {
            baseMethods,
            style: {
                musicDetailRender: {
                    transformX: 0,
                }
            },

            // UI大小调节
            UIScale_userSet: 1,
            /**
             * 
             */
            UIScale_autoSet: 1, 


            
            // 提供下方缓存保存
            dragInfo: null,

            // 组件位置状态
            /**
             * bottom: 在底部
             * top: 在顶部
             * toBottom: 从顶部动画至底部中
             * toTop：从底部动画至顶部中
             */
            musicInfoPagePosition: 'bottom',

            // 缓存监听器注销器
            eventListenerRemovers: [],
            cancelCoverBindReg: () => { },

            // 用于标识当前处于哪个时刻激活的动画
            // 理论值：Number: Date.now()
            changePositionTimeStamps: 0
        }
    },
    computed: {
        progress: function () {
            return Number((this.audioState.currentTime / this.audioState.duration).toFixed(3))
        },
        UIScale: function(){
            return ((this.UIScale_userSet != 1)?this.UIScale_userSet:this.UIScale_autoSet)+ 'rem'
        }
    },
    components: {
        lazyLoadCoverImage,
        buttom_icon_circleBackground,
        playModeSvg,
        textSpawn,
        background,
        contain
    },
    provide() {
        return {
            musicInfoPagePositionState: computed(() => this.musicInfoPagePosition)
        }
    },
    inject: ['currentMusicInfo', 'audioState', 'audioManager', 'changePlayMode', 'trackState', 'musicTrack',
        'nextMusic', 'prevMusic', 'getNextMusicIndex', 'getPrevMusicIndex', 'regResizeHandle', 'config'
    ],
    mounted() {

        // 注册底部栏拖动
        this.onBottomListener();

        // 注册进度条拖动
        ([
            this.$refs.progressBoxContainer,
            this.$refs.bar_ProgressBoxContainer
        ]).map((elm) => {
            // console.log(elm);

            baseMethods.progressBarReg(elm, () => {
                return this.audioState.currentTime / this.audioState.duration
            }, (info) => {
                if (info.draging == true) {
                    // console.log(info);
                    // Number((this.audioState.currentTime / this.audioState.duration).toFixed(3))
                    this.audioManager.audioDom.currentTime = Math.min(info.currentProgress * this.audioState.duration, this.audioState.duration - this.config.audio.audioStreamDuration - 1)
                    this.progress = info.currentProgress
                }
            })
        })
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
            // 清理现有的监听器
            this.eventListenerRemovers.forEach((value) => value());

            // 新增顶部的监听器
            this.onTopListener();
            this.musicInfoPagePosition = "toTop";

            // 记录当前时间戳，用于判断动画是否完成
            const timeStamps = Date.now();
            this.changePositionTimeStamps = timeStamps;

            // 判断是否仍为当前动画的函数
            const stillIsThisAnimation = () => this.changePositionTimeStamps === timeStamps;

            // 移动页面到顶部
            anime({
                targets: this.$refs.musicInfoPageRow,
                easing: 'spring(1, 80, 16,' + Math.abs(info.speedY).toFixed(2) + ')',
                translateY: -document.body.offsetHeight + 'px',
                complete: () => {
                    this.musicInfoPagePosition = "top"; // 动画结束时更新状态
                }
            });

            // 隐藏控制栏
            anime({
                targets: this.$refs.musicControlBar,
                opacity: 0,
                easing: 'linear',
                duration: 100
            });

            // 移动音乐详情
            anime({
                targets: this.style.musicDetailRender,
                transformX: 1,
                easing: 'linear',
                duration: 100
            });

            // 取消模糊效果
            setTimeout(() => {
                if (stillIsThisAnimation()) {
                    anime.set(this.$refs.musicInfoPageRow, {
                        background: 'rgb(233,233,233)',
                        backdropFilter: 'blur(0px)'
                    });
                }
            }, 300);

            // 显示主容器
            anime({
                targets: this.$refs.mainContainer,
                opacity: 1,
                easing: 'linear',
                duration: 100,
                delay: 100,
            });

            // 重新绑定封面图片、主界面位置
            let position_bind = (speed) => {

                anime.set(this.$refs.musicInfoPageRow, {
                    translateY: -document.body.offsetHeight + 'px',
                })
                cover_position_bind(speed)
            }
            let cover_position_bind = (speed) => {
                let positionData = this.$refs.coverImagePlaceHolder.getBoundingClientRect()

                anime({
                    targets: this.$refs.cover,
                    easing: 'spring(1, 80, 15,' + speed + ')',
                    width: positionData.width,
                    height: positionData.height,
                    translateY: (this.$refs.coverImagePlaceHolder.offsetTop + 41) + 'px',
                    translateX: positionData.x + 'px'
                })
            }
            this.$nextTick(() => { cover_position_bind(Math.abs(info.speedY).toFixed(2)) })
            this.cancelCoverBindReg = this.regResizeHandle('coverMove', () => { position_bind(0) }).cancelReg

        },

        // 将音乐信息页面移动到底部
        toBottom(info) {
            this.musicInfoPagePosition = "toBottom";

            // 取消封面图片位置绑定
            this.cancelCoverBindReg();

            // 清理现有的监听器
            this.eventListenerRemovers.forEach((value) => value());
            // 记录当前时间戳，用于判断动画是否完成
            const timeStamps = Date.now();
            this.changePositionTimeStamps = timeStamps;

            // 判断是否仍为当前动画的函数
            const stillIsThisAnimation = () => this.changePositionTimeStamps === timeStamps;

            // 添加模糊效果
            anime.set(this.$refs.musicInfoPageRow, {
                background: 'rgba(0, 0, 0, 0.0625)',
                backdropFilter: 'blur(30px)'
            });

            // 移动页面到底部
            anime({
                targets: this.$refs.musicInfoPageRow,
                easing: 'spring(1, 80, 16,' + Math.abs(info.speedY).toFixed(2) + ')',
                translateY: -88,
                complete: () => {
                    this.musicInfoPagePosition = "bottom";
                }
            });

            // 显示控制栏
            anime({
                targets: this.$refs.musicControlBar,
                opacity: 1,
                easing: 'linear',
                duration: 100,
                delay: 300,
            });

            // 隐藏主容器
            anime({
                targets: this.$refs.mainContainer,
                opacity: 0,
                easing: 'linear',
                duration: 100,
            });

            // 缩小封面图片
            anime({
                targets: this.$refs.cover,
                width: 54,
                height: 54,
                easing: 'spring(1, 80, 15,' + Math.abs(info.speedY).toFixed(2) + ')',
                translateY: 17,
                translateX: 17
            });

            // 添加底部监听器
            this.onBottomListener();
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

                    this.dragInfo = info
                },
                (info) => {

                    if (info.offsetDirectionX != 'none') {
                        this.style.musicDetailRender.transformX = info.offsetX + lastTransformX
                    }
                    // console.log(info.speedY);

                    if (info.offsetY < -100 || info.speedY < -1.5) {
                        if (navigator.vibrate) navigator.vibrate(50);
                        this.toTop(info)
                    }
                    this.dragInfo = info

                },
                (info) => {
                    // console.log('2');

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


                    this.dragInfo = null

                })

            this.eventListenerRemovers.push(callBack_drag.destroy)
        },
        onTopListener() {

            let callBack_drag = drag.create(this.$refs.cover,
                (info) => {
                    this.dragInfo = info

                },
                (info) => {
                    // if (info.offsetY > 100) {
                    if (info.offsetY < -100 || info.speedY > 1.5) {

                        if (navigator.vibrate) navigator.vibrate(50);
                        this.toBottom(info)
                    }
                    this.dragInfo = info

                },
                (info) => {

                })
            // controlTapBar
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

            <div ref="cover" style="transform: translateX(17px) translateY(17px); " class="cover">
                <lazyLoadCoverImage class="blur" :id="currentMusicInfo.al.id"></lazyLoadCoverImage>
                <lazyLoadCoverImage :id="currentMusicInfo.al.id"></lazyLoadCoverImage>
            </div>


            <div ref="bar_ProgressBoxContainer" :style="{
                '--progress': progress
            }" class="bar_ProgressBoxContainer">
                <div class="insert"></div>
            </div>
            <div ref="musicControlBar" class="musicControlBar">
                <div class="cover placeholder">
                </div>
                <div ref="barDetail" class="detail">
                    <div :style="{
                        transform: 'translateX(' + style.musicDetailRender.transformX + 'px)'
                    }" class="dragRender">
                        <div class="prev">
                            <div class="event">上一首</div>
                            <div class="name">{{ musicTrack[getNextMusicIndex()].name }}</div>
                        </div>
                        <div class="currentMusic">
                            <div class="name">
                                <textSpawn :text="currentMusicInfo.name" />

                            </div>
                            <div class="artists">
                                <span v-for="(value, index) in currentMusicInfo.ar">
                                    <textSpawn
                                        :text="value.name + ((index != currentMusicInfo.ar.length - 1) ? '/' : '')" />
                                </span>
                                <span v-if="currentMusicInfo.al.id != -2"> - </span>{{ currentMusicInfo.al.name }}

                            </div>
                        </div>
                        <div class="next">
                            <div class="event">下一首</div>
                            <div v-if="trackState.playMode != 'randomPlay'" class="name">
                                {{ musicTrack[getNextMusicIndex()].name }}</div>
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
                        @click="(audioState.playing == true) ? audioManager.pause() : audioManager.play()"
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
            <div :style="{
                'fontSize': UIScale,
                'pointer-events': (musicInfoPagePosition == 'top') ? 'auto' : 'none',
            }" ref="mainContainer" class="mainContainer">
                <background class="player-background" :coverId="currentMusicInfo.al.id"
                    :musicInfoPagePosition="musicInfoPagePosition" />

                <div class="controlBar">
                    <div ref="controlTapBar" class="tapBar"></div>
                </div>
                <div class="musicDetail">

                    <div ref="coverImagePlaceHolder" class="coverImagePlaceHolder">
                    </div>
                    <div class="musicInfo">
                        <div class="name">
                            <textSpawn :text="currentMusicInfo.name" />
                        </div>
                        <div class="artists">
                            <span v-for="(value, index) in currentMusicInfo.ar">
                                <textSpawn
                                    :text="value.name + ((index != currentMusicInfo.ar.length - 1) ? '/' : '')" />
                            </span>
                            <span v-if="currentMusicInfo.al.id != -2"> - </span>{{ currentMusicInfo.al.name }}
                        </div>

                    </div>
                    <div class="infoPage_ProgressContainer">
                        <div ref="progressBoxContainer" :style="{
                            '--progress': progress
                        }" class="progressBoxContainer">
                            <div class="progressBox">
                                <div class="progressInsert"></div>
                            </div>
                        </div>
                        <div class="progressInfo">
                            <div class="current">
                                {{ baseMethods.formatTime_MMSS(audioState.currentTime_round) }}
                            </div>
                            <div class="duration">
                                {{ baseMethods.formatTime_MMSS(audioState.duration_round) }}
                            </div>
                        </div>
                    </div>
                    <div class="musicDetailButton">
                        <buttom_icon_circleBackground @click="changePlayMode">
                            <template #icon>
                                <playModeSvg style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;">
                                </playModeSvg>
                            </template>
                        </buttom_icon_circleBackground>

                        <buttom_icon_circleBackground @click="prevMusic()">
                            <template #icon>
                                <i class="bi bi-skip-start-fill"></i>
                            </template>
                        </buttom_icon_circleBackground>
                        <buttom_icon_circleBackground
                            @click="(audioState.playing == true) ? audioManager.pause() : audioManager.play()"
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
                        <buttom_icon_circleBackground>
                            <template #icon>
                                <i style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;"
                                    class="bi-volume-up bi"></i>
                                <!-- <playModeSvg  style="transform: scale(.7) translateY(1%);transform-origin: 50% 50%;"></playModeSvg> -->
                            </template>
                        </buttom_icon_circleBackground>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
<style scoped>
.musicInfo>.name {
    font-weight: 900;
    color: #fffd;
    font-size: 1.3125em;
}

.musicInfo>.artists {
    font-size: 0.875em;
    color: #fff9
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
    width: 100%;
    word-break: break-all;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
}

.currentMusic>.artists,
.prev>.name,
.next>.name {
    font-size: 14px;
    /* font-weight: 900; */
    color: #676767;
    width: 100%;
    word-break: break-all;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
}

.player-background {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    z-index: 0;
    height: 100%
}

.musicDetail {
    position: absolute;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    box-sizing: border-box;
    padding-bottom: 40px;
    max-height: calc(100% - 48px);
    left: 50%;
    transform: translateX(-50%);
    width: min(50vh, 40vw);
}

.musicDetailButton {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.musicDetailButton>* {
    flex-shrink: 0;
    font-size: 1em;

}

.coverImagePlaceHolder {
    grid-column: info-side;
    grid-row: cover;
    /* background-color: #000; */
    align-self: center;
    justify-self: center;
    background-position: center;
    background-size: cover;
    border-radius: 1%;
    overflow: hidden;
    image-rendering: auto;
    width: min(50vh, 40vw);
    aspect-ratio: 1/1;
    height: min(50vh, 40vw);
    cursor: pointer;
}

.bar_ProgressBoxContainer {
    width: 100%;
    height: 3px;
    margin: -0px 0 0 0;
    background-color: #0002;
    box-shadow: var(--Shadow-value-low);
}

.bar_ProgressBoxContainer .insert {
    width: calc(var(--progress) * 100%);
    height: 100%;
    background-color: var(--color-toggle-actived);
    box-shadow: var(--Shadow-value-low);
}

.mainContainer {
    user-select: none;
    opacity: 0;
    font-size: var(--adaptiveSize);
    color: rgb(255, 255, 255);

}

.tapBar {
    margin: 0.6em auto;
    height: .8em;
    cursor: n-resize;
    z-index: 1;
    /* display: a; */
/* position: absolute; */
    width: 5em;
    border-radius: 1em;
    background-color: #fff2;
}

.controlBar {
    height: 2em
}

.mainContainer {
    font-size: 1rem;
    z-index: 1;
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
    /* outline: 1px solid #d6d6d6; */
    box-shadow: 0 0px 15px rgba(0, 0, 0, 0.14);
    /* transform: translateY() */
    overflow: hidden
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
    pointer-events: all
}

.cover {
    position: absolute;
    height: 54px;
    width: 54px;
    /* transform: translateX(17px) translateY(17px);  */
    z-index: 1;
    top: 0;
    left: 0;
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

.cover * {
    -webkit-user-drag: none;

    -moz-user-drag: none;

    -ms-user-drag: none;

    user-drag: none;
}

.cover>.blur {
    filter: blur(0.75em);
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

.progressBoxContainer {
    height: 1.875em;
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin-bottom: 0.125em;
}

.progressBoxContainer:hover .progressBox {
    background-color: #fff5;
    width: 104%;
    height: 1.125em;
    border-radius: 0.5625em;
    margin: 0 -2%;
}

.progressBoxContainer:hover .progressInsert {
    background-color: #fffd;

}

.infoPage_ProgressContainer .progressBox {
    position: relative;
    margin: 0 0%;
    width: 100%;
    /* width: 98%; */
    height: 0.875em;
    border-radius: 0.4375em;
    box-shadow: var(--Shadow-value-offsetY-low);
    background-color: #fff3;
    cursor: pointer;
    transition: 0.2s ease-in-out;
    overflow: hidden;
}

.infoPage_ProgressContainer .progressInsert {
    height: 100%;
    transition: background-color 0.2s ease-in-out;
    background-color: #fff7;
    width: calc(var(--progress) * 100%)
}

.infoPage_ProgressContainer>.progressInfo {
    display: flex;
    justify-content: space-between;
    font-size: 0.5625em;
    color: #fff9
}
</style>