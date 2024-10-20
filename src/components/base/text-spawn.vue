<template>
  <div class="box">
    <a ref="newtext">{{ newText }}</a>
  </div>
</template>

<script>
  import anime from 'animejs/lib/anime.es';
  export default {
    data() {
      return {
        newText: '',
        anime: false,
        waiting: false
      };
    },
    props: {
      text: String
    },
    mounted() {
      this.updateTitle(this.text)
    },
    methods: {
      updateTitle() {
        if (this.anime == true) {
          this.waiting=true
          return;
        }
        this.anime = true


        setTimeout(() => {
          this.newText = this.text;
        }, 100);

        setTimeout(() => {
          if(this.waiting==true){
            this.waiting=false
            this.updateTitle()
          }
        }, 200);

        anime({
          targets: this.$refs.newtext,
          opacity: [1, 0],
          easing: 'linear',
          duration: 100,
          complete: () => {

            anime({
              targets: this.$refs.newtext,
              opacity: [0, 1],
              easing: 'linear',
              duration: 100,
              complete:()=>{
                this.anime = false

              }
            })

          }
        })


      }
    },
    watch: {
      text: {
        handler(newValue, oldValue) {
          // 设置新文本和旧文本
          this.updateTitle()
        },
        immediate: false,
      },
    },
  };
</script>

<style scoped>
  .box {
    position: relative;
    width: inherit;
    display: inline;
  }

  .oldText {
    position: absolute;
    width: 100%;
  }
</style>