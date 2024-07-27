<template>
  <div class="image-container">
    <!-- 占位组件 -->
    <div class="placeholder">
		<i class="bi bi-music-note"></i>
	</div>
    <!-- 动态绑定的图片元素 -->
    <img
      v-if="src"
      :src="currentSrc"
      class="loaded-image"
      :style="{ opacity: imageOpacity }"
    />
	<img
	  v-if="src"
	  :src="src"
	  @load="handleImageLoad"      ref="image"
	  @error="handleImageError"
	  :style="{ opacity: 0,visibility: 'hidden' }"
	/>
  </div>
</template>

<script>

export default {
  data() {
    return {
      imageOpacity: 0, // 控制图片透明度的数据属性
	  currentSrc: '',
	  nextTransilateTime: 0,
	  timerID: undefined
    };
  },
  props: {
    src: {
      type: String,
      default: "", // 默认为空字符串，表示没有图片
    },
  },
  methods: {
    handleImageLoad(event) {
      // 当图片加载完成时，执行淡入动画
	  let delay = this.nextTransilateTime-Date.now()
		if(this.timerID!=undefined){
			clearTimeout(this.timerID)
		}
	  if(delay>=0){
		this.timerID = setTimeout(()=>{
			this.currentSrc=this.src
			this.imageOpacity = 1;
		},delay)
	  } else {
		  	this.currentSrc=this.src
		  	this.imageOpacity = 1;
	  }
	  
	  
      this.$emit("imageLoaded", event); // 触发imageLoaded事件，传递事件对象
    },
    handleImageError(error) {
      // 当图片加载失败时，可以处理错误
      console.error("Image load error:", error);
      this.$emit("imageError", error); // 触发imageError事件，传递错误对象
    },
    fadeOutImage() {
      // 当src变化时，先淡出当前图片
      this.imageOpacity = 0;
	  this.nextTransilateTime = Date.now() + 500
    },
  },
  watch: {
    src: {
      immediate: false, // 初始时立即执行一次处理函数
      handler(newSrc, oldSrc) {
        if (oldSrc !== newSrc) {
          // 当src发生变化时，先淡出当前图片
          this.fadeOutImage();
          // 在下次DOM更新循环结束之后执行，确保DOM已更新
          this.$nextTick(() => {
            // 设置图片透明度为0，准备加载新图片
            this.imageOpacity = 0;
          });
        }
      },
    },
  },
  mounted() {
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
  /* border-radius: 5%; */
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
