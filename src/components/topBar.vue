<script setup>

defineProps({
  leftBarState: String,
  titleOffsetTop: Number
})

import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()
// const count = ref(0)
</script>

<template>
  <div :class="['topbar',leftBarState]">
    <iconToClick @click="this.$router.go(-1)">
      <i class="bi bi-chevron-left"></i>
    </iconToClick>
    <div  data-tauri-drag-region class="drag" >
      <div class="blur"></div>
      <div :style="{
        '--paddingTop':titleOffsetTop+'px'
      }" data-tauri-drag-region class="title">
        <slot name="title" />
        <slot name="appname"/>
      </div> 
    </div>
    <div>
      <slot name="buttoms" />
    </div>
  </div>
</template>

<style scoped>
  .topbar{
    height: 34px;
    display: flex;
    align-items: center;
    gap: 7px;
    width: 100%;
    margin: -8px 0 0 -8px;
    z-index: 10;
    padding: 7px;
    position: relative;
    user-select: none;
    
  }

  .drag{
    -webkit-app-drag:drag;
    display: block;
    height: calc(100% + 16px);
    flex: 1;
    display: flex;
    align-items: center;
  }
  .title{
    font-size: 32px;
    padding-top: 24px;
    font-weight: 900;
    color: #222;
    transform: translateY(max(0px,calc(var(--paddingTop) - 14px)));
    padding-left: 16px;
    transition: 0.20s cubic-bezier(.5,.3,.2,1);
    width: 100%;
  }
  .blur{
    position: absolute;
    height: 100px;
    width: 100%;
    left: 52px;

    backdrop-filter: blur(12px);
    transition: 0.25s cubic-bezier(.5,.3,.2,1);
    z-index: -1;
    mask-image: linear-gradient(180deg,#000f 0%,#000f 76%,transparent 100%);
  }
  .wide .blur{
    left: 200px;
  }
  .wide .title{
    
    padding-left: 168px;

  }
</style>
