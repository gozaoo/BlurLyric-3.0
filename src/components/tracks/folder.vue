<script>
    export default{
        data() {
            return {}
        }
    }
</script>
<template>
    <div class="row">
        <i class="bi bi-folder-fill"></i>
        <div>
        <div class="name">
            <slot name="name"/>
        </div>
        <div class="path">
            <slot name="path"/>
        </div></div>
        <iconToClick @click="this.$emit('del')" class="toRight">
            <i class="bi bi-x"></i>
        </iconToClick>
        
    </div>
</template>
<style scoped>
    .row{
        background-color:rgb(0, 0, 0,0.03);
        padding: 6px 12px;
        margin: 1em 0;
        border-radius: 5px;
        display: flex;
        align-items: center;
        gap: 6px;
        overflow: hidden;
        
    }
    .bi-folder-fill{
        height: 25px;
        width: 25px;
    }
    i{
        font-size: 20px;
    }
    .toRight{
        margin: 0 0 0 auto;
    }
    .name{
        font-size: xx-small;
    }
    .path{
        font-size: small;
    }
</style>