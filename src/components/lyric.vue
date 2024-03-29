<!--lyric.vue-->
<script>
    import anime from 'animejs/lib/anime.es';
    import elemListener from '../js/elemListener';
    import lyricParser from '../js/lyricParser.js'
    import lyricLineWordbyword from './lyric-line-wordbyword.vue';
    export default {
        data() {
            return {
                config: {
                    usingwfwLyric: true, //是否启用逐字歌词
                    energySavingMode: false // 是否继续渲染歌词画面
                },
                lyric: {
                    type: 'none'
                },
                state: {
                    nowTime: 0,
                },
                intervalIDs: {
                    LyricCalculate: undefined,
                    wfwLyricCalculate: undefined
                },
                tempData: {
                    windowHeight: undefined,
                    halfWindowHeight: undefined
                },
                activeLineIndexs: [],
                centerLine: 0,
                lastActiveLineIndexs: [],
                rendingLine: {},
                windowsResizeReturn: ()=>{}
            }
        },
        components:{
            lyricLineWordbyword
        },
        methods: {
            checkConfig(config) {

            },
            // 确定是否需要激活某一行歌词
            shouldActivateLine(line, currentTime) {
                return line.startTime <= currentTime && currentTime < line.endTime;
            },
            // 确定是否需要激活某一行歌词
            waitForActivateLine(line, currentTime) {
                return line.startTime >= currentTime
            },
            updateActiveLines(currentTime) {
                const activeLines = this.lyric.lines.filter(line =>
                    this.shouldActivateLine(line, currentTime)
                );
                const activeLineIndices = activeLines.map(line => this.lyric.lines.indexOf(line));
                this.activeLineIndexs = activeLineIndices;
                if(activeLineIndices[0]==undefined){
                    
                    const waitForActiveLines = this.lyric.lines.filter(line =>
                        this.waitForActivateLine(line, currentTime)
                    );
                    const waitForActiveLinesIndices = waitForActiveLines.map(line => this.lyric.lines.indexOf(line));

                    this.centerLine = ((waitForActiveLinesIndices[0]!=undefined)?waitForActiveLinesIndices[0]:this.lyric.lines.length)
                } else {
                    this.centerLine=activeLineIndices[0]
                }
            },
            // 计算当前应该激活的歌词行
            LyricCalculate() {
                if (this.lyric.type !== 'none') {
                    this.state.nowTime = this.audioDom.currentTime+0.2;
                    this.updateActiveLines(this.state.nowTime);

                    // 如果正在渲染歌词画面，则继续
                    if (this.config.energySavingMode == false) {
                        // 计算应该显示的行和理应的赋值
                    }
                }
            },
            LyricCalculateIntervalLuncher() {
                this.intervalIDs.LyricCalculate = setInterval(() => {
                    this.LyricCalculate()
                }, 100);
            },
            deepEqual(obj1, obj2) {
                if (obj1 === obj2) return true;
                if (typeof obj1 !== "object" || obj1 === null || typeof obj2 !== "object" || obj2 === null) {
                    return false;
                }
                let keys1 = Object.keys(obj1);
                let keys2 = Object.keys(obj2);
                if (keys1.length !== keys2.length) return false;
                for (let key of keys1) {
                    if (!keys2.includes(key) || !this.deepEqual(obj1[key], obj2[key])) {
                        return false;
                    }
                }
                return true;
            },
            LyricListRender() {
                let rendingLine = {}
                let centerTop = window.innerHeight * 0.35
                let lastTop = centerTop,
                    lastBottom = centerTop
                let elements = document.querySelectorAll("div>#lyricLine"),
                    halfElementOffsetHeight = 0
                let currentCenterLine = this.centerLine 
                if (!this.lyric.lines) return
                if (this.lyric.lines[currentCenterLine] && elements.length > 0) {
                    const element = elements[currentCenterLine]
                    halfElementOffsetHeight = element.offsetHeight / 2
                    lastTop = centerTop - halfElementOffsetHeight
                    lastBottom = centerTop + halfElementOffsetHeight

                    rendingLine[currentCenterLine] = {
                        index: currentCenterLine,
                        top: centerTop - halfElementOffsetHeight
                    }
                }
                if (currentCenterLine == undefined) {
                    currentCenterLine = -1
                }
                for (let i = currentCenterLine - 1; lastBottom >= 0; i--) {
                    const element = elements[i];
                    if (element) {
                        lastTop -= element.offsetHeight
                        lastBottom -= element.offsetHeight
                        rendingLine[i] = {
                            index: i,
                            top: lastTop
                        }
                    } else {
                        lastTop = -1
                        lastBottom = -1
                    }
                }

                lastTop = centerTop - halfElementOffsetHeight
                lastBottom = centerTop + halfElementOffsetHeight

                for (let i = currentCenterLine + 1; lastTop <= this.tempData.windowHeight; i++) {
                    const element = elements[i];
                    if (element) {
                        lastTop = lastBottom
                        lastBottom += element.offsetHeight
                        rendingLine[i] = {
                            index: i,
                            top: lastTop
                        }
                    } else {
                        lastBottom = this.tempData.windowHeight
                        lastTop = this.tempData.windowHeight + 1
                    }
                }

                this.rendingLine = rendingLine
                return rendingLine
            },
            LyricLineRender(useAnimation, newrendingLine, oldrendingLine) {

                let lines = document.querySelectorAll("div>#lyricLine")
                const self = this
                let needHiddenIndex = [],
                    stillVisibleIndex = [],
                    needVisibleIndex = []

                this.tempData.windowHeight = window.innerHeight
                this.tempData.halfWindowHeight = self.tempData.windowHeight / 2
                let useMove = (newrendingLine != undefined && oldrendingLine != undefined)
                if (useMove == true) {

                    // 找出不同触发类型的元素
                    for (const key in oldrendingLine) {
                        if (Object.hasOwnProperty.call(oldrendingLine, key)) {
                            const value = oldrendingLine[key];
                            let hasSameIndex = false
                            for (const key2 in (newrendingLine)) {
                                const value2 = newrendingLine[key2];
                                hasSameIndex = (hasSameIndex == false) ? value2.index == value.index : true
                            }
                            if (hasSameIndex == true && this.lyric.lines[value.index]) {
                                stillVisibleIndex.push(value.index)
                            } else if (this.lyric.lines[value.index]) {
                                needHiddenIndex.push(value.index)
                            }
                        }
                    }
                    for (const key in newrendingLine) {
                        if (Object.hasOwnProperty.call(newrendingLine, key)) {
                            const value = newrendingLine[key];
                            let hasSameIndex = false
                            for (const key2 in (oldrendingLine)) {
                                const value2 = oldrendingLine[key2];

                                hasSameIndex = (hasSameIndex == false) ? value2.index == value.index : true
                            }
                            if (hasSameIndex == false && this.lyric.lines[value.index]) {
                                needVisibleIndex.push(value.index)
                            }
                        }
                    }
                }
                if (useAnimation == true) {
                    if (useMove == true) {
                        needHiddenIndex.forEach(info => {
                            const element = lines[info];
                            this.cssEditor(element, {
                                transform: 'translateY(100vh)',
                                visibility: 'hidden',
                            })
                        });
                        needVisibleIndex.forEach(info => {
                            const element = lines[info];
                            this.cssEditor(element, {
                                // transform: 'translateY(' + newrendingLine[info].top + 'px)',
                                visibility: 'visible',
                            })
                            let offsetStartPosition = (oldrendingLine[info])?(oldrendingLine[info].top - newrendingLine[info].top):0
                            anime({
                                targets: element,
                                // 防止歌词更新过快，而新生成的歌词直接覆盖
                                translateY: (newrendingLine[info-1])?([(newrendingLine[info].top + offsetStartPosition) + 'px',newrendingLine[info].top+'px']):([newrendingLine[info].top+'px',newrendingLine[info].top+'px']),
                                delay: (el, index, length) => {
                                    return (newrendingLine[info.index]) ? ((info.index - self
                                            .activeLineIndexs + 3) *
                                        45) : 0
                                },
                                easing: 'spring(1, 90, 14, 0)',

                            })
                        })
                        anime({
                            targets: lines,
                            translateY: (el, index, length) => {
                                return (newrendingLine[index]) ? (newrendingLine[index].top + 'px') : ('0')
                            },
                            delay: (el, index, length) => {
                                if((newrendingLine[index])){
                                    let delay = (newrendingLine[index]) ? ((index - this.centerLine + 3) *
                                        45) : 0
                                    return delay

                                } else {
                                    return 0
                                }
                            },
                            easing: 'spring(1, 90, 14, 0)',

                        })
                    }
                } else {
                    if (useMove == true) {
                        for (let i = 0; i < needHiddenIndex.length; i++) {
                            const element = lines[needHiddenIndex[i].index];
                            const info = needHiddenIndex[i]
                            this.cssEditor(element, {
                                visibility: 'hidden',
                                top: '0px'
                            })
                        }
                        for (let i = 0; i < stillVisibleIndex.length; i++) {
                            const element = lines[stillVisibleIndex[i].index];
                            const info = stillVisibleIndex[i]
                            this.cssEditor(element, {
                                visibility: 'visible',
                                top: info.top + 'px'
                            })
                        }
                        for (let i = 0; i < needVisibleIndex.length; i++) {
                            const element = lines[needVisibleIndex[i].index];
                            const info = needVisibleIndex[i]
                            this.cssEditor(element, {
                                visibility: 'visible',
                                top: info.top + 'px'
                            })
                        }
                    }
                }
            },
            // 高亮行显示
            activeLine(lines, useAnimation) {
                if (lines == undefined) lines = document.querySelectorAll("#lyricLine>.lyricTextRow")
                for (const key in this.rendingLine) {
                    if (Object.hasOwnProperty.call(this.rendingLine, key)) {
                        const element = this.rendingLine[key];
                        let hasActive = this.activeLineIndexs.findIndex((value) => element.index == value) != -1
                        if (hasActive == true && lines[element.index]) {

                            if (useAnimation == true) {
                                anime({
                                    targets: lines[element.index],
                                    color: "rgb(0, 0, 0, .6)",
                                    scale: '1',
                                    easing: 'spring(1, 80, 13, 0)',
                                })
                            } else {
                                this.cssEditor(lines[element.index], {
                                    color: "rgb(0, 0, 0, .6)",
                                    transform: lines[element.index].style.transform + ' scale(1)'
                                })
                            }
                        } else if (lines[element.index]) {
                            if (useAnimation == true) {
                                anime({
                                    targets: lines[element.index],
                                    color: "rgb(0, 0, 0,0.3)",
                                    scale: '0.9',
                                    easing: 'spring(1, 80, 13, 0)',
                                })

                            } else {
                                this.cssEditor(lines[element.index], {
                                    color: "rgb(0, 0, 0,0.3)",
                                    transform: lines[element.index].style.transform + ' scale(0.9)'
                                })
                            }

                        }
                    }
                }
            },
            cssEditor(element, style) {
                // 检查传入的是否为一个 DOM 元素
                if (!(element instanceof Element)) {
                    return;
                }
                // 检查 style 是否为一个对象
                if (typeof style !== 'object' || style === null) {
                    return;
                }
                // 如果 style 对象为空，则清除元素上的所有内联样式
                if (Object.keys(style).length === 0) {
                    element.removeAttribute('style');
                } else {
                    // 遍历 style 对象，并应用每个样式规则
                    for (const property in style) {
                        element.style.setProperty(property, style[property]);
                    }
                }
            }

        },
        beforeDestroyed: () => {
            clearInterval(this.intervalIDs.LyricCalculate);
            clearInterval(this.intervalIDs.wfwLyricCalculate);
            this.windowsResizeReturn.removeWindowsResize()
        },
        props: {
            audioDom: HTMLAudioElement,
            lyricText: Object,
            importedConfig: Object
        },
        watch: {
            lyricText: {
                handler: async function (newVal) {
                    if (this.lyricText.type == 'none') {
                        this.lyric = {
                            type: 'none'
                        };
                        return
                    }
                    this.lyric = (this.lyricText.type == "yrc") ? lyricParser.parseYRClyric(this.lyricText
                        .content) : lyricParser.parseLRClyric(this.lyricText.content)
                    this.rendingLine = {}

                    if (this.lyricText.tranContent) {
                        let tranEdLyric = lyricParser.parseLRClyric(this.lyricText.tranContent)
                        this.lyric.headers = {
                            ...this.lyric.headers,
                            ...tranEdLyric.headers
                        }
                        for (let index = 0; index < this.lyric.lines.length; index++) {
                            const element = this.lyric.lines[index];
                            let tranEdLine = tranEdLyric.lines.find((value) => {
                                return Math.abs(value.startTime - element.startTime) < 0.2
                            })
                            if (tranEdLine == undefined) continue
                            element['tranEdContent'] = tranEdLine.text
                        }
                    }
                    this.$nextTick(() => {
                        this.$nextTick(() => {
                            this.LyricListRender()
                            this.activeLine(undefined, true)

                        })
                    })
                },
                deep: true
            },
            importedConfig: {
                handler: async function (newVal) {
                    this.checkConfig()
                },
                deep: true
            },
            activeLineIndexs: {
                handler: async function (newVal, oldVal) {
                    if (!this.deepEqual(newVal, this.lastActiveLineIndexs)) {
                        this.lastActiveLineIndexs = newVal

                        this.activeLine(undefined, true)
                    }
                },
                deep: true
            },
            centerLine: {
                handler: async function (newVal, oldVal) {
                        this.LyricListRender()

                },
            },
            rendingLine: {
                handler: async function (newVal, oldVal) {
                    if (!this.deepEqual(oldVal, this.rendingLine)) {
                        this.LyricLineRender(true, newVal, oldVal)
                    }
                },
                deep: true
            }
        },
        created() {
            const self = this
            this.checkConfig()
            this.LyricCalculateIntervalLuncher()
            let cacheWindowHeight = () => {
                    this.tempData.windowHeight = window.innerHeight
                    this.tempData.halfWindowHeight = self.tempData.windowHeight / 2
            }
            this.windowsResizeReturn = elemListener.onWindowsResize(() => {

                cacheWindowHeight();
                // console.log(
                    this.LyricListRender()
                // ); 

            },0)
            this.$nextTick(cacheWindowHeight)
        }
    }
</script>
<template>
    <div ref="lyricRow" v-if="lyric&&lyric.type != 'none'&&lyric.lines" class="lyricRow">
        <div v-for="(item, index) in lyric.lines" style="transform:translateY(100vh) " :key="item.startTime" id="lyricLine"
            class="lyricLine">

            <div style="color:rgb(0, 0, 0,0);transform:scale(0.9)" :class="['lyricTextRow',(activeLineIndexs.includes(index))?'focus':'blur']">

                <div v-if="item" class="content">
                    <div v-if="lyric.type == 'lrc'">{{ item.text }}</div>
                    <div v-if="lyric.type == 'yrc'&&!activeLineIndexs.includes(index)">{{ item.text }}</div>
                    <lyricLineWordbyword :audioDom="this.audioDom" :words="item.words" v-if="lyric.type == 'yrc'&&activeLineIndexs.includes(index)" />
                </div>
                <div v-if="item&&item.tranEdContent" class="tran">
                    <div>{{ item.tranEdContent }}</div>

                </div>
            </div>

        </div>
    </div>
</template>
<style scoped>
    .lyricRow {
        display: relative;
        height: 100vh;
        mask-image: linear-gradient(180deg, transparent, #000 20%, #000 55%, transparent 100%);
    }

    .lyricLine {
        position: absolute;
        font-size: min(2.5vh, 2vw);
        padding: 0.9em 0;
        font-weight: 900;
        transition: color 0.3s;
        transform-origin: 0% 50%;
        transform: translateY(100vh);
        visibility: hidden;
        will-change: transform, visibility
    }

    .lyricTextRow {
        transform: scale(0.9);
        will-change: transform, color;
        transform-origin: 3% 50%;
        transition: .6s filter;
    }
    .lyricTextRow.blur{
        /* filter: blur(.1em) */
    }

    div.content {
        font-size: 1.8em;
    }

    div.tran {
        font-size: 1.2em;
    }
</style>