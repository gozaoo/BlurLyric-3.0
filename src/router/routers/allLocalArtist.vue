<script>
    import manager from '../../api/manager'
    import powerTable_music from '../../components/tracks/powerTable_music.vue';
    export default{
        data(){
            return {
                artistList: [

                ],
                manager
            }
        },
        components:{powerTable_music},
        created(){
            if(this.appState.runOnTauri) {
                manager.tauri.getArtists().then((res)=>{
                this.artistList = [...this.artistList,...res]

                });

                
            }
            
        },
        inject:['appState','coverMusicTrack']
    }
</script>

<template>
    <bodytitle text="全部艺人" />
    <h2>共 {{ artistList.length }} 位 </h2> 
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
        <iconWithText @click="this.$router.push({
            path: '/localArtist/',
            query: {
                id: item.id,
                type: 'local'
            }
        })" v-for="(item) in artistList" type="background" >
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