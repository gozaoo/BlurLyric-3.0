
<script setup></script>
<template>
    <div class="row">
        <slot />
    </div>
</template>
<style scoped>
    .row{
        display: flex;
        gap: 7px;
        flex-wrap: wrap;
    }
</style>
