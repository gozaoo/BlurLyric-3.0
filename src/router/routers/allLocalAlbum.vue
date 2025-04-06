<script>
import manager from '../../api/manager'
import girdRowAlbum from "../../components/gridRowAlbum.vue"
import album from '../../components/album_lazyLoad.vue'
import conditioner from '../../components/tracks/conditioner.vue'
import baseMethods from '../../js/baseMethods'
// import powerTable_music from '../../components/tracks/powerTable_,music.vue';
export default {
    methods: {
        handleConditionChange(event) {
            this.arraySortCondition = event;
            this.albumList = baseMethods.filterAndSort([...this.albumList], event);
        }
    },
    data() {
        return {
            albumList: [

            ],
            arraySortCondition: {
				// 定义condition对象
				filterFunction: (item) => {
					const content = "";
					return (
						item.name.includes(content)
					);
				},
				getKey: (item) => new String(item.track_number),
				sortOrder: 'asc', // 可选参数，'asc'或'desc'
			},
            getKeyMethods: [
                {
                    id: 1,
                    name: '专辑名',
                    method: (item) => item.name
                },
                // {
                //     id: 2,
                //     name: '歌手',
                //     method: (item) => item.artist.map(artist => artist.name).join(' ')
                // },

            ],
            manager
        }
    },
    components: { girdRowAlbum, album, conditioner },
    created() {
        if (this.appState.runOnTauri) {
            manager.tauri.getAlbums().then((res) => {
                this.albumList = [...this.albumList, ...res]
            });


        }

    },
    inject: ['appState', 'coverMusicTrack']
}
</script>

<template>
    <bodytitle text="全部专辑" />
    <h2>共 {{ albumList.length }} 张 </h2>
    <!-- <iconFlexRow>
        <iconWithText @click="manager.tauri.refreshMusicCache()" type="background" >
            <template #svg>
                <i class="bi bi-play-fill"></i>
            </template>
<template #text>
                刷新
            </template>
</iconWithText>
</iconFlexRow> -->
    <!-- <br> -->
    <conditioner class="conditioner" :condition="arraySortCondition" :getKeyMethods="getKeyMethods" @conditionChange="handleConditionChange"></conditioner>
    <girdRowAlbum>
        <album @click="this.$router.push({
            path: '/localAlbum/',
            query: {
                id: item.id,
                type: 'local'
            }
        })" v-for="(item) in albumList" :album="item"></album>
    </girdRowAlbum>
    <!-- <iconFlexRow>
        <iconWithText  @click="this.$router.push({
            path: '/localAlbum/',
            query: {
                id: item.id,
                type: 'local'
            }
        })"  v-for="(item) in albumList" type="background" >
            <template #svg>
                <i class="bi bi-person-circle"></i>
            </template>
            <template #text>
                {{ item.name}}
            </template>
        </iconWithText>
    </iconFlexRow> -->
    <!-- <powerTable_music :tableData="{
        cellArray:this.musicList
    }" /> -->
</template>

<style scoped>
.buttomTrack {
    display: flex;

}

.buttomTrack>* {
    width: fit-content;
}
</style>