<template>
	<div class="table-container">
		<!--内容标题-->
		<div class="table-name">
			<div v-for="(item,index) in currentTable.cellName" class="table-name-cell" 
			:style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}"
				
				>{{item.name}}</div>
		</div>


		<!--内容位置-->
		<div v-for="(line,line_index) in currentTable.cellArray" class="table-row">

			<div :style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}" v-for="(item,index) in currentTable.cellName" :class="['table-cell',item.type]">

				<!--内容分类-->
				<!--文本类型-->
				<span v-if="item.type=='content'||item.type=='trackOrdinalNumber'">
					{{line[item.path]}}
				</span>
				<!--图片类型-->
				<lazy-load-cover-image-vue v-if="item.type=='image'" :src='line[item.path]'
					style="left:0;top:0;height: 100%;width: 100%;position: absolute;"></lazy-load-cover-image-vue>

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

	.table-name-cell:hover {
		border-radius: 7px;
		background-color: #00000007;
		box-shadow: var(--Shadow-value-low);
		color: var(--fontColor-content-normal);
	}

	.table-row:hover {
		background-color: #0001;
		border-radius: 9px;
		box-shadow: var(--Shadow-value-normal);
	}

	.table-cell {
		position: relative;
		display: flex;
		padding: 4px;
		text-overflow: ellipsis;
		align-items: center;
		white-space: nowrap;
	}

	.table-cell.content {
		position: relative;
		display: flex;
		padding: 4px;
		text-overflow: ellipsis;
		overflow: hidden;
		align-items: center;
		white-space: nowrap;
	}

	.table-cell:hover {
		border-radius: 7px;
		background-color: #00000007;
		box-shadow: var(--Shadow-value-low);
		color: var(--fontColor-content-normal);
	}
</style>

<script>
	import lazyLoadCoverImageVue from '../base/lazyLoadCoverImage.vue'
	export default {
		data() {
			return {
				tempTableData: {},
				currentTable: {
					cellName: [{
						type: 'trackOrdinalNumber',
						path: 'trackOrdinalNumber',
						name: '#',
						sizing: 'basis',
						sizingValue: '1.75em',
						spacialStyle: {
							color: 'var(--fontColor-content-moreUnimportant)',
							fontSize: '.8em',
						}
					}, {
						type: 'image',
						path: 'imgSrc',
						name: '图像',
						sizing: 'basis',
						sizingValue: '38px'
					}, {
						type: 'content',
						path: 'name',
						name: '歌曲名',
					}, {
						type: 'content',
						path: 'artist',
						name: '歌手',
						spacialStyle: {
							color: 'var(--fontColor-content-unimportant)',
						}
					}, {
						type: 'content',
						path: 'album',
						name: '专辑',
						spacialStyle: {
							color: 'var(--fontColor-content-unimportant)',
						}
					}, {
						type: 'content',
						path: 'duration',
						name: '时长',
						sizing: 'basis',
						sizingValue: '40px',
						spacialStyle: {
							color: 'var(--fontColor-content-unimportant)',
						}
					}],
					cellArray: [{
							name: '时间线',
							artist: 'HOYO-MIX',
							album: '崩坏星穹铁道-失控 Out of Control',
							imgSrc: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',
							trackOrdinalNumber: '1',
							duration: '02:02',
						},
						{
							name: '时间线b',
							artist: 'HOYO-MaIX',
							album: 'c崩坏星穹铁道-失控 Out of Control',
							imgSrc: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',
							trackOrdinalNumber: '3',
							duration: '002',
						}
					]
				}
			}
		},
		components: {
			lazyLoadCoverImageVue
		},
		props: {
			tableData: Object,
		},
		created() {
			setTimeout(() => {
				this.currentTable.cellArray[0].imgSrc =
					"http://p2.music.126.net/USxHosVXMbDjxI1cO5-9JA==/109951169507879008.jpg"
			}, 1500)
			
			
			
		},
		methods: {}
	}
</script>