<script>
    import anime from 'animejs/lib/anime.es';

    export default {
        data() {
            return {
                animeCallBack: function () {},
                updateActiveLinesIntervalID: null,
                wordsInformationCache: [],
                lineWidthCache: 0,
                progress: '0%',
            };
        },
        methods: {
            shouldActivateWord(word, currentTime) {
                return word.startTime <= currentTime && currentTime < word.endTime;
            },
            updateActiveLines() {
                // 获取当前音频播放的时间
                const currentTime = this.audioDom.currentTime+0.3;

                const activeWordsIndex =[];
                // 在words中找出当前的词
                const activeWords = this.words.filter((word,index) => {
                    if(this.shouldActivateWord(word, currentTime)){
                        activeWordsIndex.push(index)
                        return true
                    } else {
                        return false
                    }
                });

                // 如果找到了当前的词，计算该词已经经过的百分比
                if (activeWords.length > 0) {
                    const currentWord = activeWords[0]; // 假设每个时刻只有一个词是激活的
                    const elapsed = currentTime - currentWord.startTime;
                    const percentage = elapsed / currentWord.duration;
                    const thisCache = this.wordsInformationCache[activeWordsIndex[0]]
                    // 计算相对行长度
                    const relativeLineLength = thisCache.left + thisCache.width * percentage;
                    // 根据lineWidthCache计算音乐进度移动的百分比
                    const musicProgressPercentage = relativeLineLength / this.lineWidthCache;

                    this.progress =  (musicProgressPercentage * 100).toFixed(2) + '%'
                }
            },
        },
        props: {
            words: Array, // 修改为Array，因为words应该是一个数组
            audioDom: HTMLAudioElement
        },
        created() {
            this.$nextTick(() => {
                this.words.forEach((word, index) => {
                    const element = this.$refs[word.startTime][0]; // 使用word的ref来获取DOM元素
                    if (element) {
                        this.wordsInformationCache[index] = {
                            width: element.offsetWidth,
                            left: this.lineWidthCache
                        };
                        this.lineWidthCache += element.offsetWidth;
                    }
                });

                this.updateActiveLinesIntervalID = setInterval(this.updateActiveLines, 200);
            });
        },
        beforeDestroy() {
            // 清除定时器，防止内存泄漏
            if (this.updateActiveLinesIntervalID) {
                clearInterval(this.updateActiveLinesIntervalID);
            }
        },
    };
</script>

<template>
    <div class="yrcLine" :style="{'--progress':this.progress}">
        <!-- 使用word的ref作为每个span的引用 -->
        <span :class="[
            (audioDom.currentTime + 0.3>=word.startTime)?((audioDom.currentTime+0.3<=word.endTime)?'activing':'actived'):undefined
        ]" :ref="word.startTime" v-for="word in words" :key="word.startTime">{{ word.word }}</span>
    </div>
</template>
<style scoped>
    .yrcLine {
        --progress: 0%;
        display: inline;
        background-origin: content-box;
        background-clip: text;
        /* --background-transition-linear-gradient-progress-width: 0em !important; */
        --background-transition-linear-gradient-progress-width: 0.942em !important;
        background: linear-gradient(90deg, rgb(0, 0, 0, .6) 0%, rgb(0, 0, 0, .6) calc(50% - calc(var(--background-transition-linear-gradient-progress-width)/2)), rgba(0, 0, 0, .3) calc(50% + calc(var(--background-transition-linear-gradient-progress-width)/2)), rgba(0, 0, 0, .3) 100%);
        background-size: calc(200% + var(--background-transition-linear-gradient-progress-width) * 2) 100%;
        -webkit-background-clip: text;
        background-position: calc(100% - var(--progress)) 0%;
        background-repeat: no-repeat;
        color: transparent !important;
        transition: 0.2s linear;
        /* will-change:background */
    }
    span{
    }
    span.activing{

    }
    span.activing,span.actived{
        /* display: ; */
         /* transform: translate(10px, 20px); */
         /* font-size: 0.9=em; */
}
</style>