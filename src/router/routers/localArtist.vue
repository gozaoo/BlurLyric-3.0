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
            ar: undefined,
            ar_songs: []
        };
    },
    inject: ['coverMusicTrack'],
    components: {powerTable_Music},
    watch: {
        // 监听$route对象中的params属性 
        $route: {
            handler(_result, __) {
                // 检查id或type是否发生变化
                const newParams = _result.query

                if (newParams.id != undefined && (newParams.id !== this.currentId || newParams.type !== this.currentType)) {
                    this.currentId = newParams.id;
                    this.currentType = newParams.type;
                    this.handleParamChange(newParams.id, newParams.type);
                }
            },
            deep: true, // 开启深度监听
            immediate: true, // 组件初始化时立即执行一次handler
        },
    },
    methods: {
        handleParamChange(id, type) {
            // 在这里处理id和type的变化
            // console.log(`ID changed to: ${id}, Type changed to:${type}`);
            switch (type) {
                case 'local':
                    manager.tauri.getArtistById(id).then((ar) => {
                        
                        this.ar = ar
                    })
                    manager.tauri.getArtistsSongsById(id).then((ar_songs) => {

                        this.ar_songs = ar_songs
                    })
                    break;
                case 'online':

                    break;
                default:
                    manager.tauri.getArtistById(id).then((ar) => {
                        
                        this.ar = ar
                    })
                    manager.tauri.getArtistsSongsById(id).then((ar_songs) => {

                        this.ar_songs = ar_songs
                    })
                    break;
            }

        }
    }
}
</script>

<template>
    <bodytitle v-if="ar" :text="'艺人 '+ar.name" />
    <h2>共 {{ ar_songs.length }} 首 </h2> 

    <br>
    <powerTable_Music :tableData="{
        cellArray:this.ar_songs
    }" />
</template>

<style scoped>
/* 样式内容 */
</style>
