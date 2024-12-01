<template>
  <div class="image-container">
    <!-- 占位组件 -->
    <div class="placeholder">
      <i class="bi bi-music-note"></i>
    </div>
    <!-- 动态绑定的图片元素 -->
    <img
      v-if="currentSrc"
      :src="currentSrc"
      class="loaded-image"
      :style="{ opacity: imageOpacity }"
    />
  </div>
</template>

<script>
import manager from '../../api/manager';


// import { getAlbumCover } from './manager.tauri'; // 假设manager.tauri是导入路径

export default {
  data() {
    return {
      imageOpacity: 0, // 控制图片透明度的数据属性
      currentSrc: '', // 当前图片的源
    };
  },
  props: {
    id: {
      type: Number,
      default: "", // 默认为空字符串，表示没有图片
    },
  },
  methods: {
    async loadAlbumCover() {
      try {
        if(this.id>=0){

          const base64String = await manager.tauri.getAlbumCover(this.id);
          this.currentSrc = `data:image/jpeg;base64,${base64String}`; // 假设返回的是JPEG图片
          this.imageOpacity = 1; // 图片加载成功后，设置透明度为1
        } 
      } catch (error) {
        console.error("Failed to load album cover:", error);
        this.$emit("imageError", error); // 触发imageError事件，传递错误对象
      }
    },
    handleImageLoad(event) {
      // 当图片加载完成时，执行淡入动画
      this.imageOpacity = 1;
      this.$emit("imageLoaded", event); // 触发imageLoaded事件，传递事件对象
    },
    handleImageError(error) {
      // 当图片加载失败时，可以处理错误
      this.$emit("imageError", error); // 触发imageError事件，传递错误对象
    },
  },
  watch: {
    id: {
      immediate: true, // 当albumId变化时立即执行
      handler(newAlbumId) {
        if (newAlbumId) {
          this.loadAlbumCover(); // 加载专辑封面
        }
      },
    },
  },
  mounted() {
    // 组件挂载后立即加载专辑封面
    // this.loadAlbumCover();
  },
};
</script>


<style scoped>
.image-container {
  position: relative;
  overflow: hidden;	box-shadow: var(--Shadow-value-normal);
  }
.placeholder {
	height: 100%;
	width: 100%;
	background-color: #00000007;
	color:var(--fontColor-content-moreUnimportant);
	display: flex;
	font-size: 1.3em;
	align-items: center;
	justify-content: center;
}

.loaded-image {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: auto;
  object-fit: cover; /* 保持图片的比例，不拉伸 */
  transition: opacity .5s; /* 淡入淡出动画 */
}
</style>
