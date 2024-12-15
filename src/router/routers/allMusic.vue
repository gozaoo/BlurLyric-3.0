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
            if(this.appState.runOnTauri) {
                manager.tauri.getMusicList().then((res)=>{
                this.musicList = [...this.musicList,...res]
                });
            }
            
        },
        inject:['appState','coverMusicTrack']
    }
</script>

<template>
    <bodytitle text="全部音乐" />
    <h2>共 {{ musicList.length }} 首 </h2> 
    <div class="buttomTrack">
        <iconWithText @click="coverMusicTrack(this.musicList)" type="background" >
            <template #svg>
                <i class="bi bi-play-fill"></i>
            </template>
            <template #text>
                播放全部
            </template>
        </iconWithText>
    </div>
    <br>
    <powerTable_music :tableData="{
        cellArray:this.musicList
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