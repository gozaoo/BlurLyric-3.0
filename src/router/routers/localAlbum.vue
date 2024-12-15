<script>
import manager from '../../api/manager';
import powerTable_Music from '../../components/tracks/powerTable_music.vue';
// import powerTable_Music from '../../components/tracks/powerTable_,music.vue';
export default {
    data() {
        return {
            // 可以存储id和type的值，以便在watch中进行比较
            currentId: null,
            currentType: null,
            al: {
                name: '',
                id: -2
            },
            al_songs: []
        };
    },
    components: {powerTable_Music},
    watch: {
        // 监听$route对象中的params属性 
        $route: {
            handler(_result, __) {
                // 检查id或type是否发生变化
                const newParams = _result.query

                if (newParams.id != -2 && (newParams.id !== this.currentId || newParams.type !== this.currentType)) {
                    this.currentId = newParams.id;
                    this.currentType = newParams.type;
                    this.handleParamChange(newParams.id, newParams.type);
                }
            },
            deep: true, // 开启深度监听
            immediate: true, // 组件初始化时立即执行一次handler
        },
    },
    inject: ['coverMusicTrack'],
    methods: {
        handleParamChange(id, type) {
            // 在这里处理id和type的变化
            // console.log(`ID changed to: ${id}, Type changed to:${type}`);
            switch (type) {
                case 'local':
                    manager.tauri.getAlbumById(id).then((al) => {
                        
                        this.al = al
                    })
                    manager.tauri.getAlbumsSongsById(id).then((al_songs) => {

                        this.al_songs = al_songs
                    })
                    break;
                case 'online':

                    break;
                default:
                    manager.tauri.getAlbumById(id).then((al) => {
                        
                        this.al = al
                    })
                    manager.tauri.getAlbumsSongsById(id).then((al_songs) => {

                        this.al_songs = al_songs
                    })
                    break;
            }

        }
    }
}
</script>

<template>
    <div v-if="al" >
        <bodytitle :text="''" />
        <LazyLoadCoverImage class="cover" :id="al.id" />

    </div>
    <bodytitle :text="'专辑 \n'+al.name" />
        <h2>共 {{ al_songs.length }} 首 </h2> 
    <div class="buttomTrack">
        <iconWithText @click="coverMusicTrack(this.al_songs)" type="background" >
            <template #svg>
                <i class="bi bi-play-fill"></i>
            </template>
            <template #text>
                播放全部
            </template>
        </iconWithText>
    </div>
    <br>
    <powerTable_Music :noCover="true" :tableData="{
        cellArray:this.al_songs
    }" />
</template>

<style scoped>
.cover{
    display: block;
    aspect-ratio: 1 / 1;
    max-width: 14rem;
    z-index: 100;
}
/* 样式内容 */
</style>
