<script>
    // import FlexColumnRow from '../../components/flexColumnRow.vue';
    import toggle_lineRow from '../../components/tracks/toggle-line.vue'
    import tracksRow from '../../components/tracks/tracksRow.vue'

    export default {
        data() {
            return {}
        },
        components: {
            toggle_lineRow,
            tracksRow
        },
        props: {
            state: Boolean
        },
        methods: {
            test(state){
                console.log(state);
            }
        }
    }
</script>

<template>
    <h1>设置</h1>
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
        <toggle_lineRow @changeState="test" :state="true">
            <template #icon>
                <i class="bi bi-bug-fill"></i>
            </template>
            <template #text>
                启用调试模式
            </template>
        </toggle_lineRow>
    </tracksRow>
</template>

<style scoped>
    .buttomTrack {
        display: flex;

    }

    .buttomTrack>* {
        width: fit-content;
    }
</style>