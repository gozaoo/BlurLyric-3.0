<script>
import base from "../js/base";
export default {
    name: 'scrollRow',
    data() {
        return {
            scrollTopState: {
                scrollTop: 0,
                scrollSize: 0
            }
        }
    },
    mounted() {
        // 监听滚动事件
        this.$refs.scrollRow.addEventListener('scroll', this.handleScroll);
        // 初始化时获取滚动盒的大小
        this.updateScrollSize();
    },
    beforeDestroy() {
        // 组件卸载前移除滚动事件监听器
        this.$refs.scrollRow.removeEventListener('scroll', this.handleScroll);
    },
    methods: {
        handleScroll(event) {
            // base.debounce(()=>{
                // 更新当前的滚动高度
                this.scrollTopState.scrollTop = event.srcElement.scrollTop;
                // 发送自定义事件给父组件，并传递滚动状态对象
                this.$emit('update-scroll-state', this.scrollTopState);
            // }, 60)();

        },
        updateScrollSize(event) {
            // 更新滚动盒的大小
            this.scrollTopState.scrollSize = this.$refs.scrollRow.scrollHeight;
        }
    }
}
</script>

<template>
    <div ref="scrollRow" class="row" @scroll="handleScroll">
        <div class="scroll">
            <slot />
        </div>
    </div>
</template>

<style scoped>
    .row{
        overflow: scroll;
    }
    .row::-webkit-scrollbar{
        visibility: hidden;
    }
</style>
