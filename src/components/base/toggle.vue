<script>
export default {
  props: {
    modelValue: Boolean,
    type: {
      type: String,
      default: 'normal' // 设置默认值为 'normal'
    }
  },
  emits: ['update:modelValue'],
  methods: {
    changeState() {
      if (this.type === 'unavailable') return;
      this.$emit('update:modelValue', !this.modelValue);
    }
  },
  data(){
    return {
    }
  }
}
</script>

<template>
  <div @click="changeState" :class="['box', modelValue ? 'active' : '', type]">
    <div class="point"></div>
  </div>
</template>

<style scoped>
    .box{
        background: #bbb;
        height: 20px;
        border-radius: 10px;
        box-sizing: border-box;
        box-shadow: var(--Shadow-value-normal);
        width: 40px;
        transition: .3s;
        padding: 5px;
        cursor: pointer;
    }
    
    .point{
        height: 10px;
        border-radius: 5px;
        width: 10px;
        background: #eee;
        transition: .3s;
    }
    .box.active{
        background-color:var( --color-toggle-actived);
    }
    .active .point{
        /* background: #fff; */
        margin-left: 20px;
    }
    .box.unavailable{
        cursor: not-allowed;
        background: #aaa;
    }
	.box.unavailable .point{
		background: #ccc;
	}
</style>