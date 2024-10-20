<script>

export default {
    data(){
        return {

        }
    },
    inject:[
        'musicInfoPagePositionState'
    ],
    watch:{
        musicInfoPagePositionState:{
            handler(newvalue){

            }
        }
    }
    
}
</script>
<template>
    <div class="box">
        <slot />
    </div>
</template>
<style>

</style>