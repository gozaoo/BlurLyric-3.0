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
    grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
  }
  </style>