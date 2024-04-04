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
        background-color: #0001;
        border-radius: 7px;
        display: flex;
        align-items: center;
        gap: 6px;
        overflow: hidden;

    }

</style>