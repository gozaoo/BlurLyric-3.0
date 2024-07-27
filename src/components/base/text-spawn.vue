<template>
    <div class="box">
      <a ref="oldtext" class="oldText">{{ oldText }}</a>
      <a ref="newtext">{{ newText }}</a>
    </div>
  </template>
  
  <script>
  import anime from 'animejs/lib/anime.es';
  export default {
    data() {
      return {
        newText: '',
        oldText: '',
      };
    },
    props: {
      text: String
    },
    watch: {
      text: {
        handler(newValue, oldValue) {
          // 设置新文本和旧文本
          this.newText = newValue;
          this.oldText = oldValue;
            anime({
                targets: this.$refs.oldtext,
                opacity:[1,0],
                easing: 'linear',
                duration: 100
            })
            anime({
                targets: this.$refs.newtext,
                opacity:[0,1],
                easing: 'linear',
                duration: 100
            })
        },
        immediate: true,
      },
    },
  };
  </script>
  
  <style scoped>
  .box{
    position: relative;
    width: inherit;
    display: block;
  }
  .oldText{
    position: absolute;
  }
  </style>