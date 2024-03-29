<script>
    import topBar from './components/topBar.vue'
    import rightBlock from './components/rightBlock.vue'
    import leftBar from './components/leftBar.vue'
    import musicInfoPage from './components/musicInfoPage.vue'


    export default {
        name: 'app',
        components: {
            topBar,
            leftBar,
            rightBlock,
            musicInfoPage,
        },
        data() {
            return {
                leftBarState: 'short',
                audio:document.createElement('audio'),
                currentMusicInfo: {
                    name: "",
                    id: 0,
                    ar: [{
                        id: 0,
                        name: "",
                        alias: []
                    }],
                    al: {
                        id: 0,
                        name: "",
                        picUrl: "&quot;&quot;",
                    },
                }
            }
        },
    }
</script>

<template>
    <topBar :leftBarState="leftBarState"></topBar>
    
    <div class="bottom">
        <leftBar @leftBarChange="(newState)=>{leftBarState = newState}"></leftBar>
        <rightBlock></rightBlock>
        <!-- <musicInfoPage
            :musicInfo="this.currentMusicInfo"
            :audio="this.audio"
        ></musicInfoPage> -->
    </div>


</template>

<style scoped>
    .bottom {
        /* margin-top: 5px; */
        display: flex;
        height: calc(100vh - 38px - 10px);
        width: calc(100% + 16px);
        z-index: 1;
        margin-left: -8px;
    }
</style>