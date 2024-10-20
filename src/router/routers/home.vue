<script>
import flexColumnRow from '../../components/flexColumnRow.vue';
import musicCard from '../../components/musicCard.vue';

export default {
    data() {
        return {
            title: 'Loading...',
            description: '',
            intervalId: null,
        }
    },
    components:{
        flexColumnRow,
        musicCard,
    },
    created() {
        this.updateTitleAndDescription(); // 初始化标题和描述
        this.intervalId = setInterval(this.updateTitleAndDescription, 5 * 60 * 1000); // 每5分钟更新一次标题和描述
    },
    methods: {
        updateTitleAndDescription() {
            const now = new Date();
            const hour = now.getHours();
            if (hour < 4) {
                this.title = '凌晨了';
                this.description = '星星点灯，愿你有个好梦';
            } else if (hour < 7) {
                this.title = '早上好';
                this.description = '晨光中，新页待启。';
            } else if (hour < 12) {
                this.title = '早上好';
                this.description = '让音乐唤醒一天的活力';
            } else if (hour < 18) {
                this.title = '下午好';
                this.description = '让音乐陪伴你的午后时光';
            } else {
                this.title = '晚上好';
                this.description = '放松心情，享受美妙的夜晚';
            }
        },
    },
    beforeUnmount() {
        clearInterval(this.intervalId);
    }
}
</script>

<template>
    <bodytitle :text="title" />
    <h1>
        <span style="font-size: .7em;">{{ description }}</span>
    </h1>
    <iconWithText @click="this.$router.push('/allmusic/')" type="background">
        <template #svg>
            <i class="bi bi-folder-fill"></i>
        </template>
        <template #text>
            本地音乐
        </template>
    </iconWithText>


</template>

<style scoped>
</style>
