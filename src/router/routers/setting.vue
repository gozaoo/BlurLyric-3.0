<script>
    // import FlexColumnRow from '../../components/flexColumnRow.vue';
    import toggle_lineRow from '../../components/tracks/toggle-line.vue'
    import linkLine from '../../components/tracks/link-line.vue';
    import tracksRow from '../../components/tracks/tracksRow.vue'

    export default {
        data() {
            return {}
        },
        components: {
            toggle_lineRow,
            tracksRow,
            linkLine
        },
        props: {
            state: Boolean
        },
        methods: {
            test(state) {
                console.log(state);
            }
        }
    }
</script>

<template>
    <bodytitle text="设置" />

    <h2>显示</h2>
    <tracksRow>
        <toggle_lineRow @changeState="test" :state="false">
            <template #icon>
                <i class="bi bi-battery-full"></i>
            </template>
            <template #text>
                省电模式
            </template>
        </toggle_lineRow>
    </tracksRow>
    <h2>其他</h2>
    <tracksRow>

        <linkLine @click="this.$router.push('/demo/')" :type="(leftBarState=='short')?'hidden':null">
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