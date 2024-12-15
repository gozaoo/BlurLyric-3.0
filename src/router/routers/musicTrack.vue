<script>
    import manager from '../../api/manager'
    import powerTable_music from '../../components/tracks/powerTable_music.vue';
    export default{
        data(){
            return {
                musicList: [

                ]
            }
        },
        components:{powerTable_music},
        created(){
            // if(this.appState.runOnTauri) {
            //     manager.tauri.getMusicList().then((res)=>{
            //     this.musicList = [...this.musicList,...res]
            //     });
            // }
            
        },
        inject:['musicTrack','musicTrackIndex','cleanUpMusicTrack']
    }
</script>

<template>
    <bodytitle text="当前播放列表" />

    <div class="buttomTrack">
        <iconWithText type="background" @click="cleanUpMusicTrack()" >
            <template #svg>
                <i class="bi bi-play-fill"></i>
            </template>
            <template #text>
                重置列表
            </template>
        </iconWithText>
    </div>
    <br>
    <powerTable_music :tableData="{
        cellArray:musicTrack
    }" />
</template>

<style scoped>
.buttomTrack{
    display: flex;

}
.buttomTrack>*{
    width: fit-content;
}
</style>