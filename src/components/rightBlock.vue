<script>
  import scrollRow from './scrollRow.vue'

  export default {
    data() {
      return {
        scrollTopState: {
          scrollTop: 0,
          scrollSize: 0
        }
      }
    },
    components: {
      scrollRow
    },
    watch:{
    }
  }
</script>

<template>
  <scrollRow @update-scroll-state="(value)=>{
    this.scrollTopState = value
  }" class="router">
    <router-view />
  </scrollRow>

</template>

<style scoped>
  .router {
    display: block;
    background-color: var(--mainBlockBackground);
    min-width: calc(100vw - 50px);
    padding: 10px 16px;
    box-sizing: border-box;
    /* padding-top: 48px; */
    height: calc(100vh + 17px);
    margin-top: -48px;
    position: relative;
  }
</style>