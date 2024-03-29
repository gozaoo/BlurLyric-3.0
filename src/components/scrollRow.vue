<script>
    
    export default {
        name: 'scrollRow',
        data() {
            return {
            }
        },
        created(){
            this.$nextTick(()=>{
                this.$refs.scrollRow.addlisen
            })
        }
    }
</script>
<template>
    <div class="row">
        <div ref="scrollRow" class="scroll">
            <slot />

        </div>
    </div>
</template>
<style scoped>
    .row{
        overflow: scroll;
        height: calc(100vh - 90px);
        mask-image: linear-gradient(180deg, transparent, #000 10px, #000 calc(100vh - 60px),transparent calc(100vh - 50px));
    }
    .row::-webkit-scrollbar{
        visibility: hidden;
    }
</style>