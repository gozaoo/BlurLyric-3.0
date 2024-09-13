<script>

    export default {
        data(){
            return {}
        },
        components:{
        }
    }
</script>
<template>
    <div class="background">
        <div class="container">
            <slot  />
        </div>
    </div>
</template>
<style>
    .background{
        position: fixed;
        left: 0;
        top: 0;
        right: 0;
        bottom: 0;
        background-color: #0001;
        z-index: 200;
    }
</style>