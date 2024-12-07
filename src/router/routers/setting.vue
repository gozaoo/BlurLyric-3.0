<script>
    import FlexColumnRow from '../../components/flexColumnRow.vue';
    import toggle_lineRow from '../../components/tracks/toggle-line.vue'
    import linkLine from '../../components/tracks/link-line.vue';
    import tracksRow from '../../components/tracks/tracksRow.vue'

    export default {
        data() {
            return {
                localConfig: {

                }
            }
        },
        components: {
            toggle_lineRow,
            tracksRow,
            linkLine
        },
        methods: {
            refreshConfig(){
                this.localConfig = this.config;
            }
        },
        inject:['config','editConfig'],
        created(){
            this.refreshConfig()
        },
        watch:{
            config:{
                handler: (newVal)=>{
                    this.refreshConfig()
                }
            },
            localConfig:{
                deep:true,
                handler:function(){
                    console.log(this.localConfig);
                    
                    this.editConfig(()=>{
                        return this.localConfig
                    })
                    
                }
            }
        }
    }
</script>

<template>
    <bodytitle text="设置" />

    <!-- <h2>显示</h2>
    <tracksRow>
        <toggle_lineRow v-model="test">
            <template #icon>
                <i class="bi bi-battery-full"></i>
            </template>
            <template #text>
                省电模式
            </template>
        </toggle_lineRow>
    </tracksRow>  -->
    <h2>播放偏好</h2>
    <tracksRow>
        <toggle_lineRow v-model="localConfig.audio.smartStreamAudioList">
            <template #icon>
                <i class="bi bi-water"></i>
            </template>
            <template #text>
                交叉过渡
            </template>
        </toggle_lineRow>
    </tracksRow>
    <h2>其他</h2>
    <tracksRow>

        <linkLine @click="this.$router.push('/demo/')">
            <template #icon>
                <i class="bi bi-bug-fill"></i>
            </template>
            <template #text>
                进入调试面板
            </template>
        </linkLine>
    </tracksRow>
    <bodytitle text="关于" />
    <h2>BlurLyric</h2>

</template>

<style scoped>
    .buttomTrack {
        display: flex;

    }

    .buttomTrack>* {
        width: fit-content;
    }
</style>