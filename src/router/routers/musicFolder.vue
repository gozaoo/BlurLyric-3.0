<script>
import folder from '../../components/tracks/folder.vue';
import dialog from '../../components/base/dialog.vue';
import dialog_vue from '../../components/base/dialog.vue';
import powerTable from '../../components/tracks/powerTable.vue';
import manager from '../../api/manager';
export default {
    data() {
        return {
            manager,
            localFolders: [],
            askAddLocalDirs: false,
            addLocalDirInputValue: ""
        }
    },
    components: {
        dialog,
        dialog_vue,
        powerTable,
        folder
    },
    computed: {
        onlineSourceTableData() {
            let tableData = {
                cellName: [{
                    type: 'content',
                    path: 'name',
                    name: '名称',
                }, {
                    type: 'content',
                    path: 'apiUrl',
                    name: 'api链接',
                }, {
                    type: 'content',
                    path: 'type',
                    name: '来源类型',
                }],
                cellArray: this.source.online,
            }

            return tableData
        },
    },
    methods: {
        async refreshDirs() {
            let result = await manager.tauri.getAllMusicDirs()
            this.localFolders = result;
        }
    },
    async created() {
        this.refreshDirs()
    },
    inject: ['appState', 'source']
}
</script>

<template>
    <bodytitle text="音乐来源" />
    <div v-if="appState.runOnTauri == true">

        <h2>管理本地文件夹</h2>
        <iconFlexRow>

            <iconWithText @click="manager.tauri.refreshMusicCache(); refreshDirs()" type="background">
                <template #icon>
                    <i class="bi bi-arrow-clockwise"></i>
                </template>
                <template #text>刷新</template>
            </iconWithText>
            <iconWithText @click="askAddLocalDirs= true" type="background">
                <template #icon>
                    <i class="bi bi-plus-circle-dotted"></i>
                </template>
                <template #text>添加</template>
            </iconWithText>
            <iconWithText @click="this.$router.push('/allmusic/')" type="background">
                <template #svg>
                    <i class="bi bi-folder-fill"></i>
                </template>
                <template #text>
                    全部音乐
                </template>
            </iconWithText>        <iconWithText @click="this.$router.push('/allLocalArtist/')" type="background">
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
        <dialog_vue v-if="askAddLocalDirs == true" :cancel="()=>{askAddLocalDirs = false}" :finish="()=>{askAddLocalDirs = false;manager.tauri.addMusicDirs(addLocalDirInputValue);manager.tauri.refreshMusicCache(); refreshDirs()}">
            <h2>
                请输入一个地址
            </h2>
            <p class="tips">示例：C:\Users\gozaoo\Music\</p>
            <!-- <br> -->
            <input style="width: 210px" type="text" placeholder="" v-model="addLocalDirInputValue">
        </dialog_vue>
        <folder @del="()=>{manager.tauri.removeMusicDirs(item);manager.tauri.refreshMusicCache(); refreshDirs()}" v-for="(item) in localFolders">
            <template #name>
                <!-- {{item.split('/')[-1]}} -->
            </template>
            <template #path>
                {{ item }}
            </template>

        </folder>
        <p class="tips" v-if="localFolders.length == 0">当前还没有添加的目录，请先添加</p>
    </div>
    <h2>在线来源</h2>
    <iconWithText type="background">
        <template #icon>
            <i class="bi bi-plus-circle-dotted"></i>
        </template>
        <template #text>添加</template>
    </iconWithText> <br>
    <powerTable :tableData="onlineSourceTableData"></powerTable>

</template>

<style scoped>
.buttomTrack {
    display: flex;

}

.buttomTrack>* {
    width: fit-content;
}
</style>