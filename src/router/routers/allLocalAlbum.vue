<script>
    import manager from '../../api/manager'
    // import powerTable_music from '../../components/tracks/powerTable_,music.vue';
    export default{
        data(){
            return {
                albumList: [

                ],
                manager
            }
        },
        // components:{powerTable_music},
        created(){
            if(this.appState.runOnTauri) {
                manager.tauri.getAlbums().then((res)=>{
                this.albumList = [...this.albumList,...res]
                });

                
            }
            
        },
        inject:['appState','coverMusicTrack']
    }
</script>

<template>
    <bodytitle text="全部专辑" />
    <h2>共 {{ albumList.length }} 张 </h2> 
    <!-- <iconFlexRow>
        <iconWithText @click="manager.tauri.refreshMusicCache()" type="background" >
            <template #svg>
                <i class="bi bi-play-fill"></i>
            </template>
            <template #text>
                刷新
            </template>
        </iconWithText>
    </iconFlexRow> -->
    <br>
    <iconFlexRow>
        <iconWithText  @click="this.$router.push({
            path: '/localAlbum/',
            query: {
                id: item.id,
                type: 'local'
            }
        })"  v-for="(item) in albumList" type="background" >
            <template #svg>
                <i class="bi bi-person-circle"></i>
            </template>
            <template #text>
                {{ item.name}}
            </template>
        </iconWithText>
    </iconFlexRow>
    <!-- <powerTable_music :tableData="{
        cellArray:this.musicList
    }" /> -->
</template>

<style scoped>
.buttomTrack{
    display: flex;

}
.buttomTrack>*{
    width: fit-content;
}
</style>