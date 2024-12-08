<script>
    export default {
        data() {
            return{}

        }
    }
</script>
<template>
    <div class="row">
        <slot/>
    </div>
</template>
<style scoped>
    .row {
        background-color: rgba(0, 0, 0, 0.03);
        border-radius: 7px;
        display: flex;
        align-items: center;
        gap: 6px;
        overflow: hidden;
        margin-top: 6px;
    }

</style>