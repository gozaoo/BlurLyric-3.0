<template>
    <div :class="['button', { actived }]">
      <slot />
    </div>
  </template>
  
  <script>
  export default {
    props: {
      actived: {
        type: Boolean,
        default: false
      }
    }
  };
  </script>
  
  <style scoped>
  .button {
    border: none;
    border-radius: 0.3125em;
    width: fit-content;
    aspect-ratio: 1/1;
    height: 1em;
    color: white;
    padding: 0.5em 0.8em;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: rgba(255, 255, 255, 0.1);
    transition: 0.3s;
  }
  .button.actived {
    background-color: rgba(255, 255, 255, .86);
    position: relative;
    overflow: hidden;
    z-index: 1;
    color: #000;

  }
  .button:hover {
    box-shadow: var(--Shadow-value-low);
    background-color: rgba(255, 255, 255, 0.18);
  }

  .button.actived:hover {
    background-color: rgba(255, 255, 255, 1);
  }
  </style>