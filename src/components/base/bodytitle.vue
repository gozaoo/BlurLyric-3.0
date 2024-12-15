<script>
    export default{
        data() {
            return {
                height: 0,
                offsetTop: 0,
                hiddenTop: 70,
                cancelReg: null
            }
        },
        inject: ['scrollState','regTitle'],
        mounted(){
            this.offsetTop = this.$refs.title.offsetTop
            this.height = this.$refs.title.offsetHeight
            this.cancelReg = this.regTitle( this.text,this.offsetTop,this.hiddenTop).cancelReg

        },
        props: {
            text: String
        },
        watch:{
            text: {
                handler(newv){
                    this.cancelReg();
                    this.cancelReg = this.regTitle( newv,this.offsetTop,this.hiddenTop).cancelReg

                }
            }
        },
        methods:{

        },
        beforeUnmount(){
            this.cancelReg()
        }
    }
</script>
<template>
    <h1 :class="['title',(offsetTop -  scrollState.scrollTop >= hiddenTop)?'display':'hidden']" ref="title">
        {{ text }}  
    </h1>
</template>

<style scoped>
    .title{
        font-size: 32px;
        transition: color 0.25s cubic-bezier(.5,.3,.2,1);
        font-weight: 900;

    }
    .title.hidden{
        color: #fff0;
    }
</style>