<script setup>
import iconToClick from './iconToClick.vue'
import scrollRow from './scrollRow.vue'


</script>

<template>
  <scrollRow  class="router">
    <router-view></router-view>
  </scrollRow>

</template>

<style scoped>
  .router{
    display: block;
    background-color: var(--mainBlockBackground);
    flex: 1;
    height: 100%;
    width: calc(100% + 7px);
    padding: 10px 16px;
    /* border-radius: 8px 0 0 0; */
    position: relative;
  }
</style>
