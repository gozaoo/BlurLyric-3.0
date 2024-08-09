<script>
import toggle from '../toggle.vue';

    export default {
        data() {
            return{}
        },
        components:{
            toggle
        },
        props:{
            state:Boolean
        },
        methods:{
        }
    }
</script>
<template>
    <div class="row">
        <slot name="icon" />

        <div>
            <div class="name">
                <slot name="text" />
            </div>
        </div>
        <i class="bi bi-chevron-right flexRight"></i>
    </div>
</template>
<style scoped>

    .row {
        /* background-color: #0001; */
        height: 32px;
        padding: 6px 12px;
        display: flex;
        user-select: none;
        align-items: center;
        gap: 6px;
        width: 100%;
        overflow: hidden;

    }

    i {
        font-size: 20px;
        height: 25px;
        width: 25px;
    }

    .name {
        font-size: small;
    }

    .path {
        font-size: small;
    }
    .flexRight{
        margin-left: auto;
        color:#0007
    }
</style>