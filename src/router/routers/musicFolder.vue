<script>
    import folder from '../../components/tracks/folder.vue';
    import dialog from '../../components/base/dialog.vue';
    import powerTable from '../../components/tracks/powerTable.vue';
    
    export default {
        data(){
            return {
                localFolders: [
                    {
                        name: "用户音乐文件",
                        path: "C:\\Users\\Administrator\\Documents\\My Music"
                    }
                ]
            }
        },
        components:{
            dialog,
            powerTable,
            folder
        },
        computed:{
            onlineSourceTableData(){
                let tableData ={
                    cellName: [{
						type: 'content',
						path: 'name',
						name: '名称',
					},{
						type: 'content',
						path: 'apiUrl',
						name: 'api链接',
					},{
						type: 'content',
						path: 'type',
						name: '来源类型',
					}],
                    cellArray: this.source.online,
                }

                return tableData
            },
        },
        inject:['appState','source','runOnTauri']
    }
</script>

<template>
    <bodytitle text="音乐来源" />
<div v-if="appState.runOnTauri == true">
    
    <h2>管理本地文件夹</h2>
    <iconFlexRow>

        <iconWithText type="background">
            <template #icon>
                <i class="bi bi-arrow-clockwise"></i>
            </template>
            <template #text>扫描文件夹</template>
        </iconWithText> 
        <iconWithText type="background"> 
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
        </iconWithText>
    </iconFlexRow>
    
    <br>
    <folder v-for="(item) in localFolders">
        <template #name>
            {{item.name}}
        </template>
        <template #path>
            {{item.path}}
        </template>

    </folder>
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
.buttomTrack{
    display: flex;

}
.buttomTrack>*{
    width: fit-content;
}
</style>