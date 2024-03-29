<script>
import { onMounted } from 'vue'
    import lyricVue from "../lyric.vue";

    export default {
        data() {

        },
        components:{
            lyricVue,
        },
        methods:{
            
            formTime(sec) { //秒数转化为mm:ss
                let s = sec % 60 < 10 ? ('0' + sec % 60) : sec % 60
                let min = Math.floor(sec / 60) < 10 ? ('0' + Math.floor(sec / 60)) : Math.floor(sec / 60)
                return min + ':' + s
            },
        },
        props:{
            musicInfo: Object,
            controls: Object,
            playerState: Object,
            lyric: Object,
            audio: HTMLAudioElement
        },
        created(){
            this.$nextTick(()=>{
            this.$emit('bindRef',{path:'menu',element:this.$refs.menu})
            this.$emit('bindRef',{path:'coverImagePlaceHolder',element:this.$refs.coverImagePlaceHolder})
        

            })},
        onMounted(){

        }

    }
</script>
<template>
    <div>

        <div style="opacity:0" ref="menu" class="menu">

            <div ref="coverImagePlaceHolder" class="coverImage"
                style="image-rendering: auto; transition: background-image 0.5s linear 0s, box-shadow 0.5s ease 0s, transform 0.5s cubic-bezier(0.3, 0.2, 0.2, 1.4) 0s;">
            </div>
            <div class="musicInfo">
                <div style="display: flex;">
                    <div style="display: flex; flex: 1 1 0%; flex-direction: column; min-width: 0px;">
                        <div class="musicname">{{ musicInfo.name }}</div>


                        <div class="artists">
                            <span v-for="(item,index) in musicInfo.ar"> <span>{{ item.name }}</span> <span
                                    v-if="musicInfo.ar.length -1 != index">&</span> </span>
                        </div>

                    </div><button type="button" class="moreButtom"><i class="bi bi-three-dots"></i></button>
                </div>
                <div class="progressControl">
                    <div id="audioProgress" class="slider ">
                        <div :style="{'--musicProgressPercent': (playerState.icurrentTime / playerState.durationTime)}"
                            class="inner">
                            <div class="thumb"></div>
                        </div>
                    </div>
                    <div class="progressTips">
                        <div>{{formTime(playerState.icurrentTime)}}</div>
                        <div>{{formTime(playerState.durationTime)}}</div>
                    </div>
                </div>
                <div class="musicControls">
                    <button style="font-size: 1.2em"><i class="bi bi-shuffle"></i></button>
                    <button @click="controls.prevTrack()" style="font-size: 1.3em"><i
                            class="bi bi-skip-backward-fill"></i></button>
                    <button @click="controls.play()" style="font-size: 2.3em" class="am-music-play">
                        <i :class="['bi',(playerState.audioState)?'bi-pause-fill':'bi-play-fill']"></i>
                    </button>
                    <button @click="controls.nextTrack()" style="font-size: 1.3em"><i
                            class="bi bi-skip-forward-fill"></i></button>
                    <button style="font-size: 1.2em">
                        <svg style="height:1.1em;width:1.1em" t="1657018656868" class="icon" viewBox="0 0 1024 1024"
                            version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1849">
                            <path
                                d="M928 476.8c-19.2 0-32 12.8-32 32v86.4c0 108.8-86.4 198.4-198.4 198.4H201.6l41.6-38.4c6.4-6.4 12.8-16 12.8-25.6 0-19.2-16-35.2-35.2-35.2-9.6 0-22.4 3.2-28.8 9.6l-108.8 99.2c-16 12.8-12.8 35.2 0 48l108.8 96c6.4 6.4 19.2 12.8 28.8 12.8 19.2 0 35.2-12.8 38.4-32 0-12.8-6.4-22.4-16-28.8l-48-44.8h499.2c147.2 0 265.6-118.4 265.6-259.2v-86.4c0-19.2-12.8-32-32-32zM96 556.8c19.2 0 32-12.8 32-32v-89.6c0-112 89.6-201.6 198.4-204.8h496l-41.6 38.4c-6.4 6.4-12.8 16-12.8 25.6 0 19.2 16 35.2 35.2 35.2 9.6 0 22.4-3.2 28.8-9.6l105.6-99.2c16-12.8 12.8-35.2 0-48l-108.8-96c-6.4-6.4-19.2-12.8-28.8-12.8-19.2 0-35.2 12.8-38.4 32 0 12.8 6.4 22.4 16 28.8l48 44.8H329.6C182.4 169.6 64 288 64 438.4v86.4c0 19.2 12.8 32 32 32z"
                                p-id="1850"></path>
                            <path d="M544 672V352h-48L416 409.6l16 41.6 60.8-41.6V672z" p-id="1851"></path>
                        </svg>
                    </button></div>


            </div>
            <lyricVue class="lyricRow" :importedConfig="{
    usingwfwLyric: true,
    energySavingMode: false
}" :audioDom="audio" :lyricText="lyric" />
        </div>
    </div>
</template>
<style scoped>
    .menu {
        display: grid;
        width: 100%;
        height: calc( 100vh - 2*var(--adaptiveSize));
        position: relative;
        grid-template-rows: [drag-area] minmax(30px, 1fr) [thumb] auto [cover] auto [music-info] 3fr 1fr;
        grid-template-columns: [info-side] .75fr [player-side] 1fr;
        gap: 8px;
        /* transition: .5s ease-in-out; */
        font-size: var(--adaptiveSize);
        left: 0%;
        z-index: 0
    }

    .lyricRow {
        grid-column: player-side;
        grid-row: drag-area;
        margin-top: calc(-1 * var(--adaptiveSize));
    }

    .coverImage {
        color: rgb(255, 255, 255);
        aspect-ratio: 1/1;
        grid-column: info-side;
        grid-row: cover;
        align-self: center;
        justify-self: center;
        margin-top: var(--adaptiveSize);
        margin-bottom: calc( - var(--adaptiveSize));
        width: min(50vh, 40vw);
        height: min(50vh, 40vw);
        background-position: center;
        background-size: cover;
        border-radius: 1%;
        overflow: hidden;
        background-image: url("");
        image-rendering: auto;
        transition: background-image 0.5s linear 0s, box-shadow 0.5s ease 0s, transform 0.5s cubic-bezier(0.3, 0.2, 0.2, 1.4) 0s;
    }

    .musicInfo {
        display: flex;
        grid-row: music-info;
        width: min(50vh, 40vw);
        flex-direction: column;
        margin: 1em auto;
        color: #000e
    }

    .musicInfo .musicname,
    .musicInfo .moreButtom {
        font-size: 1.2em;
        font-weight: 700;

    }

    .musicInfo .musicname {
        display: flex;
        /* align-items: center; */
        overflow: hidden;
        text-overflow: ellipsis;
        -webkit-line-clamp: 2;
        display: -webkit-box;
        -webkit-box-orient: vertical;
    }

    .musicInfo .artists {
        color: #0009;
        font-weight: 700;
        overflow: hidden;
        text-overflow: ellipsis;
        -webkit-line-clamp: 1;
        display: -webkit-box;
        -webkit-box-orient: vertical;
    }

    .musicControls {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    button {
        border: 0;
        position: relative;
        background: none;
        aspect-ratio: 1/1;

    }

    button>*:hover::after {
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

    .slider {
        margin: 1em 0 0.6em 0;
        height: 0.6em;
        border-radius: 0.4em;
        background-color: #0001;
        transition: 0.2s;
        box-shadow: rgba(0, 0, 0, 0.1) 0px 0px 1.2em;
        overflow: hidden
    }

    .slider .inner {
        width: calc(var(--musicProgressPercent) * 100%);
        height: inherit;
        background-color: #000a;
    }

    .progressControl:hover>.slider {
        margin: 0.8em 0 0.4em 0;
        height: 1em;
        border-radius: 1em;
    }

    .progressTips {
        display: flex;
        color: #0008;
        font-weight: 900;
        font-size: 0.6em;
        flex-direction: row;
        justify-content: space-between
    }
    .menuSelecterBar{
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>