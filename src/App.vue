<script>
    import topBar from './components/topBar.vue'
    import rightBlock from './components/rightBlock.vue'
    import leftBar from './components/leftBar.vue'
    import musicInfoPage from './components/musicInfoPage.vue'
    import { computed } from 'vue'

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
                audio: document.createElement('audio'),
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
                },
                scrollState: {
                    scrollTop: 0,
                    scrollSize: 0
                },
                title: '主页',
                titleOffsetTop: 0
            }
        },
        provide() {
            return {
                // 提供一个名为user的数据
                scrollState:computed(() =>this.scrollState) 
            };
        },
        methods: {
            setTitle(value){
                this.title = value
            }
        },
        watch: {
        }
    }
</script>

<template>
    <topBar :titleOffsetTop="titleOffsetTop" :leftBarState="leftBarState">
        <template #title>
            <textspawn :text="this.title"/>
        </template>
    </topBar>

    <div class="bottom">
        <leftBar @leftBarChange="(newState)=>{leftBarState = newState}">
            <template #buttons>
                <!--音乐库-->
                <iconWithText @click="this.$router.push('/')" :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-house-fill"></i>
                    </template>
                    <template #text>
                        音乐库
                    </template>
                </iconWithText>

                <!--音乐目录-->
                <iconWithText @click="this.$router.push('/musicFolder/')" :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-folder-fill"></i>
                    </template>
                    <template #text>
                        音乐来源
                    </template>
                </iconWithText>

                <iconWithText @click="this.$router.push('/setting/')" :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-gear-fill"></i>
                    </template>
                    <template #text>
                        设置
                    </template>
                </iconWithText>
                <iconWithText @click="this.$router.push('/demo/')" :type="(leftBarState=='short')?'hidden':null">
                    <template #svg>
                        <i class="bi bi-bug-fill"></i>
                    </template>
                    <template #text>
                        组件测试
                    </template>
                </iconWithText>
            </template>
        </leftBar>
        <rightBlock @update-scroll-state="(value)=>{this.scrollState = value;
        }"></rightBlock>

    </div>


</template>

<style scoped>
    .bottom {
        display: flex;
        height: calc(100vh - 48px);
        width: fit-content;
        z-index: 1;
        margin-left: -8px;
    }
</style>