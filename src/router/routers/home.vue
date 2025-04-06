<script>
import flexColumnRow from '../../components/flexColumnRow.vue';
import musicCard from '../../components/musicCard.vue';
import album from '../../components/album.vue'
import girdRowAlbum from "../../components/gridRowAlbum.vue"
import manager from '../../api/manager';

export default {
    data() {
        return {
            title: 'Loading...',
            description: '',
            intervalId: null,
            randomAlbum: [],
        }
    },
    components: {
        flexColumnRow,
        musicCard,
        girdRowAlbum, album

    },
    created() {
        this.updateTitleAndDescription(); // 初始化标题和描述
        this.intervalId = setInterval(this.updateTitleAndDescription, 5 * 60 * 1000); // 每5分钟更新一次标题和描述
    },
    async mounted() {
        console.log(this.source);
        // 从本地数据this.source.local.albums.data中，不打乱原数组地随机选择4张专辑
        this.randomAlbum = this.source.local.albums.data.sort(() => Math.random() - 0.5).slice(0, 4);
        // 监听专辑数据缓存更新事件，当专辑数据缓存更新时，重新随机选择4张专辑
        manager.tauri.onCacheUpdate('albums', () => {
            this.randomAlbum = this.source.local.albums.data.sort(() => Math.random() - 0.5).slice(0, 4);
        });
        
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
    },
    inject: ['source'],
}
</script>

<template>
    <bodytitle :text="title" />
    <h1>
        <span style="font-size: .7em;">{{ description }}</span>
    </h1>
    <!-- <iconFlexRow>

        <iconWithText @click="this.$router.push('/allmusic/')" type="background">
            <template #svg>
                <i class="bi bi-folder-fill"></i>
            </template>
            <template #text>
                本地音乐
            </template>
        </iconWithText>
        <iconWithText @click="this.$router.push('/allLocalArtist/')" type="background">
            <template #svg>
                <i class="bi bi-person-circle"></i>
            </template>
            <template #text>
                所有本地艺人
            </template>
        </iconWithText>
        <iconWithText @click="this.$router.push('/allLocalAlbum/')" type="background">
            <template #svg>
                <i class="bi bi-disc-fill"></i>
            </template>
            <template #text>
                所有本地专辑
            </template>
        </iconWithText>
    </iconFlexRow>
    <h1>
        <span style="font-size: .7em;">听听专辑</span>
       
    </h1> -->

    <girdRowAlbum >
        <album @click="this.$router.push({
            path: '/localAlbum/',
            query: {
                id: item.id,
                type: 'local'
            }
        })" v-for="(item) in randomAlbum" :noTitle="true" :album="item"></album>
    </girdRowAlbum>
</template>

<style scoped></style>
