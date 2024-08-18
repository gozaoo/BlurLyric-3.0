<script>
    import lazyLoadCoverImage from './base/lazyLoadCoverImage.vue';
    import buttom_icon_circleBackground from './base/buttom_icon_circleBackground.vue';
    import playModeSvg from './musicInfoPageComponents/playModeSvg.vue';
    import drag from '../js/drag';
    import textSpawn from './base/text-spawn.vue';
import anime  from 'animejs';
    export default {
        data() {
            return {
                style: {
                    musicInfoPageRow: {
                        transformY: -88
                    },
                    musicDetailRender:{
                        transformX: 0,
                    }
                },
                eventListenerRemovers:[],
                dragInfo: null
            }
        },
        components: {
            lazyLoadCoverImage,
            buttom_icon_circleBackground,
            playModeSvg,
            textSpawn
        },
        inject: ['currentMusicInfo', 'audioState', 'audioManager', 'changePlayMode','trackState'],
        mounted(){
            let musicControlBar_animeJsCallBack = null
            let lastTransformX
            let callBack_drag =  drag.create(this.$refs.musicControlBar,
            (info)=>{
                if(info.offsetDirectionX != 'none'){
                    if(musicControlBar_animeJsCallBack!=null) musicControlBar_animeJsCallBack.pause()
                    lastTransformX = this.style.musicDetailRender.transformX
                }
                this.dragInfo = info
            },
            (info)=>{
                if(info.offsetDirectionX != 'none'){
                    this.style.musicDetailRender.transformX = info.offsetX + lastTransformX
                }

                this.dragInfo = info

            },
            (info)=>{
                
                musicControlBar_animeJsCallBack = anime({
                    targets: this.style.musicDetailRender,
                    transformX: 0,
                    easing:'spring(1, 80, 14,0)'
                }
                )

                this.dragInfo = null

            })

            this.eventListenerRemovers.push(callBack_drag.destroy)
        },
        beforeUnmount(){
            this.eventListenerRemovers.map((value)=>{value()})
        }
    }
</script>
<template>
    <div :style="{
        transform: 'translateY('+style.musicInfoPageRow.transformY+'px)'
        }" class="global_backgroundblur_light musicInfoPageRow">

        <div class="relativeBox">
            
            <!-- <div class="cover">
                    <lazyLoadCoverImage class="blur" :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                    <lazyLoadCoverImage :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                </div> -->
            <div ref="musicControlBar" class="musicControlBar">
                <div class="cover">
                    <lazyLoadCoverImage class="blur" :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                    <lazyLoadCoverImage :src="currentMusicInfo.al.picUrl"></lazyLoadCoverImage>
                </div>
                <div class="detail"> 
                    <div :style="{
                        transform:'translateX('+style.musicDetailRender.transformX+'px)'
                    }" class="dragRender">
                        <div class="prev">
                            <div class="event">上一首</div>
                            <div class="name">{{currentMusicInfo.name}}</div>
                        </div>
                        <div class="currentMusic">
                            <div class="name">
                                <textSpawn :text="currentMusicInfo.name" />
                            
                            </div>
                            <div class="artists"> 
                                <span v-for="(value,index) in currentMusicInfo.ar">
                                    <textSpawn :text="value.name + ((index!=currentMusicInfo.ar.length-1)?'/':'')" /> 
                                </span> - {{  currentMusicInfo.al.name }}
                                
                            </div>
                        </div>
                        <div class="next">
                            <div class="event">下一首</div>
                            <div v-if="trackState.playMode != 'randomPlay'" class="name">{{currentMusicInfo.name}}</div>
                            <div v-if="trackState.playMode == 'randomPlay'" class="name">随机播放</div>
                        </div>
                    </div>
                </div>
                <div class="control">
                    <buttom_icon_circleBackground>
                        <template #icon>
                            <i class="bi bi-skip-start-fill"></i>
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground class="playButtom">
                        <template #icon>
                            <div style="transform: scale(1.5) translateX(5%);transform-origin: 50% 50%;">
                                <i class="bi bi-play-fill"></i>
                            </div>
                        </template>
                    </buttom_icon_circleBackground>
                    <buttom_icon_circleBackground>
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
                </div>
            </div>
        </div>
    </div>
</template>
<style scoped>
    .musicInfoPageRow {
        position: absolute;
        height: 100%;
        width: 100%;
        left: 0;
        top: 100%;
        background-color: #0001;
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
        position: relative;
        height: 88px;
        width: 100%;
        box-sizing: border-box;
        display: flex;
        gap: 17px;
        padding: 17px;
        overflow: hidden;
    }

    .cover {
        height: 54px !important;
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
        padding: 0 10px;
        user-select: none;
        mask-image: linear-gradient(90deg,#0000 0%,#000f 10px,#000f calc(100% - 10px),#0000 100%)

    }
    
    .detail>.dragRender{
        position: absolute;
        display: flex;
        width: inherit;
    }  
    .prev{
        position: absolute;
        text-align: end;
        transform: translateX(calc(-100% - 10px))
    }
    .next{
        transform: translateX(-10px);
        position: absolute;
        width: max-content;
        left: 100%;
    }
    .prev .event,.next .event{
        font-weight: 900;
        font-size: 18px;
        color: #0066cd;
    }
    .currentMusic>.name {
        font-size: 18px;
        font-weight: 900;
        color: #333;
    }

    .currentMusic>.artists,.prev>.name,.next>.name {
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
    @media screen and (max-width: 560px){
        .control>*{
            display: none;
        }
        .control>*.playButtom{
            display: inline-flex;
        }
    }
</style>