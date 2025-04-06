<template>
    <div class="grid-container">
      <slot></slot>
    </div>
  </template>
  
  <script>
  export default {
    name: 'SlotGrid'
  }
  </script>
  
  <style scoped>
  .grid-container {
    display: grid;
    gap: 36px 48px;
    /* width: ; */
    grid-template-columns: repeat(auto-fit, minmax(130px,1fr));
  }
  </style>