<template>
	<div class="table-container">
		<!--内容标题-->
		<div class="table-name">
			<div v-for="(item,index) in currentTable.cellName" class="table-name-cell" :style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}">{{item.name}}</div>
		</div>

		<contextMenu v-for="(line,line_index) in currentTable.cellArray" :menuItems="[
			{ iconClass:['bi','bi-play-circle'], name: '插入单曲', handleClick: () => { console.log('点击了菜单项1'); } },
			{
				iconClass: ['bi','bi-music-note-list'],
				name:'其他播放操作',
				type:'hoverMenu',
				menuItems: [
					{ iconClass:['bi','bi-vinyl'], name: '播放该列表', handleClick: () => { console.log('点击了菜单项1'); } },
					{ iconClass:['bi','bi-skip-forward-circle'], name: '下一首播放', handleClick: () => { console.log('点击了菜单项1'); } },
					{ iconClass:['bi','bi-plus-circle'], name: '加入播放列表末', handleClick: () => { console.log('点击了菜单项1'); } },
				]
			},
			{
				iconClass: ['bi','bi-clipboard'],
				name:'复制内容...',
				type:'hoverMenu',
				menuItems: [
					{ iconClass:['bi','bi-justify-left'], name: '一键复制',  handleClick: () => { console.log('点击了菜单项1'); }},
					{ iconClass:['bi','bi-music-note'], name: '复制 音乐名',  handleClick: () => { console.log('点击了菜单项1'); }},
					{ iconClass:['bi','bi-image'], name: '复制 图片链接',  handleClick: () => { console.log('点击了菜单项1'); } },
					{ iconClass:['bi','bi-people'], name: '复制 创作者名',  handleClick: () => { console.log('点击了菜单项1'); } },
					{ iconClass:['bi','bi-disc'], name: '复制 专辑名 ',  handleClick: () => { console.log('点击了菜单项1'); } },]
			},
			{ type: 'hr' },
			{
				iconClass:['bi','bi-arrow-down-up'],
				name:'排列方式',
				type:'hoverMenu',
				menuItems: [
					{  name: '默认',  handleClick: () => { console.log('点击了菜单项1'); }},
					{  name: '#',  handleClick: () => { console.log('点击了菜单项1'); } },
					{  name: '歌名',  handleClick: () => { console.log('点击了菜单项1'); }},
					{  name: '歌手',  handleClick: () => { console.log('点击了菜单项1'); } },
					{  name: '专辑',  handleClick: () => { console.log('点击了菜单项1'); } },
					{  name: '时长',  handleClick: () => { console.log('点击了菜单项1'); } },
				]
			},
			{type:'hr'},
			{
				iconClass: ['bi','bi-info-circle'],
				name:'音乐详情',
				type:'hoverMenu',
				menuItems: [
					{ iconClass:['bi','bi-image'], name: '查看专辑图片',  handleClick: () => { console.log('点击了菜单项1'); } },
					{ iconClass:['bi','bi-people'], name: '音乐人 '+line.artist,  handleClick: () => { console.log('点击了菜单项1'); } },
					{ iconClass:['bi','bi-disc'], name: '专辑 '+line.album,  handleClick: () => { console.log('点击了菜单项1'); } },]
			},
			{ iconClass:['bi','bi-file-earmark-music'], name: '转存音乐文件',  handleClick: () => { console.log('点击了菜单项1'); } },
			{ iconClass:['bi','bi-share'], name: '分享',  handleClick: () => { console.log('点击了菜单项1'); } },
		]
		">
			<!--内容位置-->
			<div class="table-row">

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
		</contextMenu>
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

	.table-cell:hover {
		border-radius: 7px;
		background-color: #00000007;
		box-shadow: var(--Shadow-value-low);
		color: var(--fontColor-content-normal);
	}
</style>

<script>
	import contextMenu from '../base/contextMenu.vue';
	import base from '../../js/base';
	import lazyLoadCoverImageVue from '../base/lazyLoadCoverImage.vue'
	export default {
		inject: ['scrollState'],
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
						name: 'error 未导入任何音乐',
						artist: 'HOYO-MIX',
						album: '崩坏星穹铁道-失控 Out of Control',
						imgSrc: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',
						trackOrdinalNumber: '1',
						duration: '02:02',
					}]
				}
			}
		},
		components: {
			lazyLoadCoverImageVue,
			contextMenu,
			base
		},
		props: {
			tableData: Object,
		},
		methods: {
			copy: base.copy
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