<script>
    import manager from '../../api/manager'
    export default{
        data(){
            return {
                musicList: [

                ]
            }
        },
        created(){
            if(runOnTauri) {
                manager.tauri.getMusicList().then((res)=>{
                this.musicList = [...this.musicList,...res]
                });
            }
            
        }
    }
</script>

<template>
    <bodytitle text="全部音乐" />

    <div class="buttomTrack">
        <iconWithText type="background" @click="" >
            <template #svg>
                <i class="bi bi-play-fill"></i>
            </template>
            <template #text>
                播放全部
            </template>
        </iconWithText>
    </div>
    {{ musicList }}
</template>

<style scoped>
.buttomTrack{
    display: flex;

}
.buttomTrack>*{
    width: fit-content;
}
</style>