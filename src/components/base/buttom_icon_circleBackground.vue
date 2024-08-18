<template>
    <div class="row">
        <slot name="icon"></slot>
    </div>
    
</template>
<style scoped>
    .row{
        display: block;
        padding: 0.2em;
        align-items: center;
        justify-content: center;
        display: flex;
        aspect-ratio: 1 / 1;
        border-radius: 50%;
    }
    .row:hover{
        background-color: #0001;
    }
</style>