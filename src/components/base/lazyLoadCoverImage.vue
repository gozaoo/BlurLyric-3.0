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
    <!-- <img
      v-if="currentSrc"
      :src="currentSrc"
      @load="handleImageLoad"
      @error="handleImageError"
      :style="{ opacity: 0, visibility: 'hidden' }"
    /> -->
  </div>
</template>

<script>
import manager from '../../api/manager';
export default {
  data() {
    return {
      imageOpacity: 0, // 控制图片透明度的数据属性
      currentSrc: '', // 存储当前的ObjectURL
      nextTransilateTime: 0,
      timerID: undefined,
      objectURL: null, // 存储ObjectURL，以便于后续销毁
    };
  },
  props: {
    id: {
      type: [String, Number], // id可以是字符串或数字
      default: "", // 默认为空字符串，表示没有id
    },
  },
  methods: {
    handleImageLoad(event) {
      // 当图片加载完成时，执行淡入动画
      let delay = this.nextTransilateTime - Date.now();
      if (this.timerID !== undefined) {
        clearTimeout(this.timerID);
      }
      if (delay >= 0) {
        this.timerID = setTimeout(() => {
          this.imageOpacity = 1;
        }, delay);
      } else {
        this.imageOpacity = 1;
      }
      this.$emit("imageLoaded", event); // 触发imageLoaded事件，传递事件对象
    },
    handleImageError(error) {
      // 当图片加载失败时，可以处理错误
      this.$emit("imageError", error); // 触发imageError事件，传递错误对象
    },
    fadeOutImage() {
      // 当id变化时，先淡出当前图片
      this.imageOpacity = 0;
      this.nextTransilateTime = Date.now() + 500;
      setTimeout(() => {
        this.fetchAlbumCover();
      }, 500);
    },
    async fetchAlbumCover() {
      // 获取专辑封面并更新currentSrc
      if (this.objectURL) {
        URL.revokeObjectURL(this.objectURL); // 销毁之前的ObjectURL
      }
      manager.tauri.getAlbumCover(this.id).then(url=>{
        this.objectURL = url; // 更新ObjectURL
        this.currentSrc = url;
        this.handleImageLoad()
      }).catch((err)=>{
        if(err != 'Album cover not found'){ // 找不到专辑图片为正常现象
          console.log(err);
        }
      });

    },
  },
  watch: {
    id: {
      immediate: true, // 初始时立即执行一次处理函数
      handler(newId, oldId) {
        if (newId !== oldId&&newId>=0) {
          this.fadeOutImage();
          
        }
      },
    },
  },
  unmounted() {
    // 组件注销前，销毁ObjectURL
    if (this.objectURL) {
      URL.revokeObjectURL(this.objectURL);
    }
  },
  mounted() {
    if (this.id>=0) {
          this.fadeOutImage();
          this.fetchAlbumCover();
        }
    // 确保容器占满整个画幅
    // const container = this.$el.querySelector('.image-container');
    // container.style.width = '100%';
    // container.style.height = '100vh'; // 或者根据需要设置为100%
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
