<template>
	<div class="table-container">
		<!--内容标题-->
		<div ref="table_name" class="table-name">
			<div v-for="(item,index) in currentTable.cellName" class="table-name-cell" :style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}">{{item.name}}</div>
		</div>

		<contextMenu @click="doubleClick(line_index)" v-for="(line,line_index) in currentTable.cellArray" :menuItems="[
			{ iconClass:['bi','bi-play-circle'], name: '插入单曲', handleClick: () => { pushMusic(line) } },
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
			<div v-if="shouldDisplayIndexRange[1]>=line_index" class="table-row">

				<div :style="{
					['flex'+((item.sizing)?('-'+item.sizing):'')]: (item.sizingValue)?item.sizingValue:1,
					...item.spacialStyle
				}" v-for="(item,index) in currentTable.cellName" :class="['table-cell',item.type]">
					<div class="relativeBox">
						<!--内容分类-->
						<!--文本类型-->
						<span v-if="item.type=='content'||item.type=='trackOrdinalNumber'">
							<!-- {{  -->
								<!-- // (typeof(item.path)) -->
								<!-- // }} -->
								<!-- {{ item.path() }} -->
							{{item.path.apply({
								line,line_index,item,index
							})}}
						</span>
						<!--图片类型-->
						<lazy-load-cover-image-vue v-if="item.type=='image'&&shouldDisplayIndexRange[0]<=line_index" :id='item.path.apply({
								line,line_index,item,index
							})'
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
						type: 'trackOrdinalNumber',
						path: function(){
							return this.line_index+1;
						},
						name: '#',
						sizing: 'basis',
						sizingValue: '1.75em',
						spacialStyle: {
							color: 'var(--fontColor-content-moreUnimportant)',
							fontSize: '.8em',
						}
					}, {
						type: 'image',
						path:function(){
							return (this.line.al.name != 'Unknown Album')?this.line.al.id:-2;
						},
						name: '图像',
						sizing: 'basis',
						sizingValue: '38px'
					}, {
						type: 'content',
						path:function(){
							return this.line.name;
						},
						name: '歌曲名',
					}, {
						type: 'content',
						path:function(){
							let first = false
							return this.line.ar.map((ar,ar_index)=>{
								// if(ar_index == this.line.ar.length-1){
									return ar.name
								// }
								// return ar.name+'&'
							}).join("&");
						},
						name: '歌手',
						spacialStyle: {
							color: 'var(--fontColor-content-unimportant)',
						}
					}, {
						type: 'content',
						path:function(){
							return this.line.al.name;
						},
						name: '专辑',
						spacialStyle: {
							color: 'var(--fontColor-content-unimportant)',
						}
					}
					// ,
					//  {
					// 	type: 'content',
					// 	path:function(){
					// 		return this.line_index+1;
					// 	},
					// 	name: '时长',
					// 	sizing: 'basis',
					// 	sizingValue: '40px',
					// 	spacialStyle: {
					// 		color: 'var(--fontColor-content-unimportant)',
					// 	}
					// }
				],
					cellArray: [{
                    name: '时间线',
                    id: 0,
                    artist: 'HOYO-MIX',
                    album: '崩坏星穹铁道-失控 Out of Control',
                    duration: '02:02',
                    trackOrdinalNumber: '1',
                    imgSrc: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',

                    ar: [{
                        id: -2,
                        name: 'HOYO-MIX',
                        alias: []
                    }],
                    lyric: {
                        type: 'yrc',
                        lines: [{
                            startTime: 0,
                            duration: 2,
                            endTime: 2,
                            words: [{
                                    startTime: 0,
                                    duration: 1,
                                    endTime: 1,
                                    word: 'Hello '
                                },
                                {
                                    startTime: 1,
                                    duration: 0.5,
                                    endTime: 1.5,
                                    word: 'World'
                                }
                            ],
                            text: 'Hello World'
                        }]
                    },
                    al: {
                        id: -2,
                        name: '崩坏星穹铁道-失控 Out of Control',
                        picUrl: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',
                    },
                    src: null
                }]
				},
				lastClick:{
					index: 0,
					timeStamp: Date.now()
				},
				shouldDisplayIndexRange: [-1,12]
			}
		},
		components: {
			lazyLoadCoverImageVue,
			contextMenu,
		},
		props: {
			tableData: Object,
			noCover: Boolean
		},
		mounted(){
			if(this.noCover == true){
				this.currentTable.cellName.splice(1,1)
			}
			this.freshShouldDisplay()
		},
		methods: {
			copy: baseMethods.copy,
			doubleClick(line_index){
				let newTimestamp = Date.now();
				// console.log("pushed",this.currentTable.cellArray[line_index]);

				if(this.lastClick.index == line_index&&newTimestamp - this.lastClick.timeStamp<300){
					this.pushMusic(this.currentTable.cellArray[line_index]);
					// console.log("pushed",this.currentTable.cellArray[line_index]);
					
				}
				this.lastClick.index = line_index
				this.lastClick.timeStamp = newTimestamp
			},
			freshShouldDisplay(){
				const gap = 4;
				const oneTrackHeight =54; 
				const firstTop =gap + this.$refs.table_name.offsetTop + this.$refs.table_name.offsetHeight
				const scrollTop = this.scrollState.scrollTop;
				const bodyHeight = document.body.offsetHeight;

				// 计算第一个可见行的索引
				let couldBeSeeFirstIndex = Math.floor((scrollTop - firstTop) / (oneTrackHeight + gap));
				// 计算最后一个可见行的索引
				let couldBeSeeFinalIndex = Math.floor((scrollTop - firstTop + bodyHeight) / (oneTrackHeight + gap)) + 4;
				// 确保索引不会小于0
				couldBeSeeFirstIndex = couldBeSeeFirstIndex < 0 ? 0 : couldBeSeeFirstIndex;
				if(this.shouldDisplayIndexRange[0]!=couldBeSeeFirstIndex||this.shouldDisplayIndexRange[1]!=couldBeSeeFinalIndex) {this.shouldDisplayIndexRange = [couldBeSeeFirstIndex,couldBeSeeFinalIndex]}
				
				// console.log(this.shouldDisplayIndexRange);
			}
		},
		watch: {
			scrollState: {
				deep: true,
				handler(newValue, oldValue) {
					this.freshShouldDisplay()
				},
			},
			tableData: {
				handler(newValue) {
					if (newValue&&newValue.cellName != undefined) {
						this.currentTable = newValue
					} else if (newValue&&newValue.cellArray != undefined) {
						this.currentTable.cellArray = newValue.cellArray
					}
				},
				deep: true,
				immediate: true
			}
		},
		inject: ['scrollState','pushMusic','pushMusicTrack','coverMusicTrack','cleanUpMusicTrack']
	}
</script>