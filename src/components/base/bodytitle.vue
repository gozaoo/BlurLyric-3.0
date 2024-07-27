<script>
    import app from '../../main.js'
    export default{
        data() {
            return {
                height: 0,
                offsetTop: 0,
                hiddenTop: 70
            }
        },
        inject: ['scrollState'],
        mounted(){
            this.offsetTop = this.$refs.title.offsetTop
            this.height = this.$refs.title.offsetHeight
            this.check()
        },
        props: {
            text: String
        },
        watch:{
            scrollState:{
                handler(newvalue){
                    this.check()
                },
                deep: true,

            }
        },
        methods:{
            check(){
                if(this.offsetTop -  this.scrollState.scrollTop < this.hiddenTop){
                        app._instance.data.title = this.text
                        app._instance.data.titleOffsetTop = this.offsetTop -  this.scrollState.scrollTop
                    }
            }
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