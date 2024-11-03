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
        <div class="svg">
            <slot name="svg"></slot>
            <slot name="icon"></slot>
        </div>
        
        <div class="text" :style="(type == 'hidden')?'color: #0000;--delay:0s':null">
            <slot name="text"></slot>
        </div>
        

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
        min-height: 36px;
    }
    .buttom:hover{
        background-color: #00000008;
    }
    .text{
        white-space: nowrap;
        /* color: #000e; */
        /* color: */
        --delay:0.125s;
        padding-left: 5px;
        transition: all 0.125s cubic-bezier(.5,.3,.2,1) var(--delay)
    }
</style>