<script>
    // import folder from '../../components/tracks/folder.vue';
    import dialog from '../../components/base/dialog.vue';
    import powerTable from '../../components/tracks/powerTable.vue';
    export default {
        data(){
            return {
                
            }
        },
        components:{
            dialog,
            powerTable
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
            }
        },
        inject:['appState','source']
    }
</script>

<template>
    <bodytitle text="音乐来源" />
<div v-if="appState.isTauri == true">
    
    <h2>本地文件夹</h2>
    <!-- <folder>
        <template v-slot:name>
                用户音乐目录
        </template>
        <template v-slot:path>
                C:\Users\Administrator\Music\
        </template>
    </folder><br> -->
</div>
    <h2>在线来源</h2>
    <iconWithText type="background">
        <template #icon>
            <i class="bi bi-plus-circle-dotted"></i>
        </template>
        <template #text>增加</template>
    </iconWithText> <br>
    <powerTable :tableData="onlineSourceTableData"></powerTable>
    <!-- <folder>
        <template v-slot:name>
                用户音乐目录
        </template>
        <template v-slot:path>
                C:\Users\Administrator\Music\
        </template>
    </folder> -->
</template>

<style scoped>
.buttomTrack{
    display: flex;

}
.buttomTrack>*{
    width: fit-content;
}
</style>