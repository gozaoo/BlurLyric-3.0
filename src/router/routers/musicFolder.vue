<script setup>
    import folder from '../../components/tracks/folder.vue';
</script>

<template>
    <bodytitle text="音乐来源" />

    <h2>本地文件夹</h2>
    <folder>
        <template v-slot:name>
                用户音乐目录
        </template>
        <template v-slot:path>
                C:\Users\Administrator\Music\
        </template>
    </folder>
</template>

<style scoped>
.buttomTrack{
    display: flex;

}
.buttomTrack>*{
    width: fit-content;
}
</style>