<script>
  import scrollRow from './scrollRow.vue'

  export default {
    data() {
      return {}
    },
    props:{
      leftBarState: String
    },
    components: {
      scrollRow
    },
    watch:{
    }
  }
</script>

<template>
  <scrollRow style="padding-bottom: 100px;" :class="[leftBarState]" class="router">
    <router-view />
  </scrollRow>

</template>

<style scoped>
  .router {
    display: block;
    background-color: var(--mainBlockBackground);
    width: calc(100vw - 52px);
    min-width:  calc(100vw - 52px);
    padding: 10px 16px;
    box-sizing: border-box;
    /* padding-top: 48px; */
    height: calc(100vh + 17px);flex-shrink:0;
    margin-top: -48px;
    position: relative;
  }

</style>