<script>
    export default {
        data() {
            return {currentState: false}
        },
        props:{
            state:Boolean
        },
        methods:{
            changeState(){
                this.currentState= !this.currentState
                this.$emit('changeState',this.currentState)
            }
        },
        created(){
            this.currentState = this.state;
        },
        watch:{
            state:{
                handler(){
                    this.currentState = state
                }
            }
        }
    }
</script>

<template>
    <div @click="changeState" :class="['box',currentState?'active':'']">
        <div class="point"></div>
    </div>
</template>

<style scoped>
    .box{
        background: #bbb;
        height: 20px;
        border-radius: 10px;
        box-sizing: border-box;
        box-shadow: 0px 0px 3px #0005;
        width: 40px;
        transition: .3s;
        padding: 5px;
        cursor: pointer;
    }
    
    .point{
        height: 10px;
        border-radius: 5px;
        width: 10px;
        background: #eee;
        transition: .3s;
    }
    .box.active{
        background-color:var( --color-toggle-actived);
    }
    .active .point{
        /* background: #fff; */
        margin-left: 20px;
    }
</style>