<script setup>
    defineProps({
        type: String,
        customColor: String
    })
    const typeToStyle = {
        background: {
            backgroundColor:'#00000008',
            // boxShadow: 'var(--Shadow-value-low)'
        },
        buttom_active: {
            backgroundColor:'var(--color-button-active)',
            color:'#fff',
            boxShadow: 'var(--Shadow-value-low)'
        },

    }
</script>
<template>
    <div :style="typeToStyle[type]" class="buttom">
        <slot></slot>

    </div>
</template>

<style scoped>
    .buttom{
        height: 36px;
        width: fit-content;
        display: flex;
        gap: 7px;
        align-items: center;
        border-radius: 7px;
        box-sizing: border-box;
        padding: 0 11px ;
        user-select: none;
        cursor: pointer;
        overflow: hidden;
  transition: all 0.15s ease-in-out;
  min-height: 36px;
    }
    .buttom:hover{
        background-color: #00000008;
    }
</style>