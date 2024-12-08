<script>
    import FlexColumnRow from '../../components/flexColumnRow.vue';
    import toggle_lineRow from '../../components/tracks/toggle-line.vue'
    import linkLine from '../../components/tracks/link-line.vue';
    import tracksRow from '../../components/tracks/tracksRow.vue'

    export default {
        data() {
            return {
                localConfig: {

                },
                test: false,
                test_true: true
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

    <h2>显示</h2>
    <tracksRow>
        <toggle_lineRow :type="'unavailable'" v-model="test_true">
            <template #icon>
                <i class="bi bi-back"></i>
            </template>
            <template #text>
                使用毛玻璃效果 (**当前功能不可用)
            </template>
        </toggle_lineRow>
    </tracksRow>    
    <tracksRow>
        <toggle_lineRow :type="'unavailable'" v-model="test">
            <template #icon>
                <i class="bi bi-film"></i>
            </template>
            <template #text>
                锁定复杂画面计算帧数（24fps）
            </template>
        </toggle_lineRow>
    </tracksRow>
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
    <h2>操作偏好</h2>
    <tracksRow>
        <toggle_lineRow v-model="test">
            <template #icon>
                <i class="bi bi-water"></i>
            </template>
            <template #text>
                双击音乐列表操作
            </template>
        </toggle_lineRow>
    </tracksRow>
    <h2>基本（敬请期待）</h2>
    <tracksRow>
        <toggle_lineRow  :type="'unavailable'" v-model="test">
            <template #icon>
                <i class="bi bi-translate"></i>
            </template>
            <template #text>
                语言 | Language (**当前功能不可用)
            </template>
        </toggle_lineRow>
    </tracksRow>
    <tracksRow>
        <toggle_lineRow :type="'unavailable'" v-model="test">
            <template #icon>
                <i class="bi bi-pci-card-sound"></i>
            </template>
            <template #text>
                音频设备 (**当前功能不可用)
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