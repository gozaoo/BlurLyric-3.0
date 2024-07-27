<script setup>
import iconToClick from './iconToClick.vue'

import {ref,watch} from 'vue'

const emit = defineEmits(['leftBarChange'])

const state = ref('short')

import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

watch(
  state,
  async newState =>{
    emit('leftBarChange', newState); // 触发一个自定义事件input，并将inputText的值作为参数传递给父组件
  }
)
</script>

<template> 
  <div :class="['leftBar',state]">
    <!--调整侧栏状态-->
    <iconToClick @click="state= (state=='short')?'wide':'short'">
      <i class="bi bi-list"></i>
    </iconToClick>

    <slot name="buttons"/>
    
    <div v-if="state=='wide'" class="title">
      BLURLYRIC
    </div>
  </div>
</template>

<style scoped>
.leftBar{
  --width: 52px;
  width: var(--width);
  transition:  0.25s cubic-bezier(.5,.3,.2,1);
  user-select: none;
  display: flex;
  flex-direction: column;
  padding: 50px 7px 0 7px;
  margin-top: -50px;
  z-index: 1;
  background-color: #eee;
  gap: 7px;flex-shrink:0;
  overflow: hidden;
  max-height: 100vh;
  /* min-width: 54px; */
  box-sizing: border-box;
  position: relative;
}
.leftBar::-webkit-scrollbar{
  display: none;
}
.leftBar.wide{
  --width: 200px

}

.title{
  position: absolute;
  bottom: 10px;
  font-weight: 900;
  color: #aaa;
  left: 50%;
  font-size: 8px;
  transform: translateX(-50%);
}

</style>
