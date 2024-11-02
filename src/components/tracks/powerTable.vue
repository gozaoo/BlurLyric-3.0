<template>
	<div class="table-container">
		<!--内容标题-->
		<div class="table-name">
			<div v-for="(item,index) in currentTable.cellName" class="table-name-cell" :style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}">{{item.name}}</div>
		</div>

			<!--内容位置-->
			<div  v-for="(line,line_index) in currentTable.cellArray"  class="table-row">

				<div :style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}" v-for="(item,index) in currentTable.cellName" :class="['table-cell',item.type]">
					<div class="relativeBox">
						<!--内容分类-->
						<!--文本类型-->
						<span v-if="item.type=='content'||item.type=='trackOrdinalNumber'">
							{{line[item.path]}}
						</span>
						<!--图片类型-->
						<lazy-load-cover-image-vue v-if="item.type=='image'" :src='line[item.path]'
							style="border-radius: 5%;left:0;top:0;height: 100%;width: 100%;position: absolute;">
						</lazy-load-cover-image-vue>

					</div>
				</div>

			</div>
		<!-- 其他行 -->
	</div>
</template>

<style scoped>
	.table-container {
		display: flex;
		width: 100%;
		border-collapse: collapse;
		flex-direction: column;
		justify-content: space-between;
		gap: 4px;
	}

	.table-row {
		display: flex;
		height: 54px;
		padding: 4px;
		box-sizing: border-box;
		gap: 3px;
	}

	.table-name {
		background-color: #00000007;
		border-radius: 9px;
		display: flex;
		box-shadow: var(--Shadow-value-low);
		color: var(--fontColor-content-unimportant);
		height: 40px;
		gap: 3px;
		padding: 4px;
		box-sizing: border-box;
	}

	.table-name-cell {
		display: flex;
		align-items: center;
		padding: 4px;
	}

	/* .table-name-cell:hover {
		border-radius: 7px;
		background-color: #00000007;
		box-shadow: var(--Shadow-value-low);
		color: var(--fontColor-content-normal);
	} */

	/* .table-row:hover {
		background-color: #0001;
		border-radius: 9px;
		box-shadow: var(--Shadow-value-normal);
	} */

	.table-cell {
		display: flex;
		padding: 4px;
		text-overflow: ellipsis;
		position: relative;
		align-items: center;
		white-space: nowrap;
	}

	.relativeBox {}

	.table-cell.content {
		display: flex;
		padding: 4px;
		text-overflow: ellipsis;
		overflow: hidden;
		align-items: center;
		white-space: nowrap;
	}

	/* .table-cell:hover {
		border-radius: 7px;
		background-color: #00000007;
		box-shadow: var(--Shadow-value-low);
		color: var(--fontColor-content-normal);
	} */
</style>

<script>
	import contextMenu from '../base/contextMenu.vue';
	import baseMethods from '../../js/baseMethods';
	import lazyLoadCoverImageVue from '../base/lazyLoadCoverImage.vue'
	export default {
		inject: ['scrollState'],
		data() {
			return {
				tempTableData: {},
				currentTable: {
					cellName: [{
						type: 'content',
						path: 'text',
						name: '没有任何东西',
					}],
					cellArray: [{
						path: '请向组件传递正确内容',
					}]
				}
			}
		},
		components: {
			lazyLoadCoverImageVue,
			contextMenu,
		},
		props: {
			tableData: Object,
		},
		methods: {
			copy: baseMethods.copy
		},
		watch: {
			scrollState: {
				handler(newValue, oldValue) {
					console.log(newValue);
				}
			},
			tableData: {
				handler(newValue) {
					if (newValue.cellName != undefined) {
						this.currentTable = newValue
					} else if (newValue.cellArray != undefined) {
						this.currentTable.cellArray = newValue.cellArray
					}
				},
				deep: true,
				immediate: true
			}
		}
	}
</script>