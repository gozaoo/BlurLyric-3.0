<script>
            import config from '../js/config.js'

    export default{
        data(){
            return{
                anime: undefined,
                config,
                position:[
                  {},
                  {},
                  {},
                  {}
                ],
                dynFunctionRunning:false
            }
        },
        created(){
          this.position = []
            for (let i = 0; i < 4; i++) {
              this.position.push({
                '--random-x': (Math.random() * 100) + '%',
                '--random-y':(Math.random() * 100) + '%'
              })
            }
            setTimeout(() => {

            if(this.dynFunctionRunning ==false&&this.mainDisplay!='buttom'&&config.setting().config.useAnimeBackground){
                  this.random()
            }
          },3000)

        },
        methods:{
          random(){
            this.dynFunctionRunning = true
            this.position = []
            for (let i = 0; i < this.$refs.block.length + 1; i++) {
              this.position.push({
                '--random-x': (Math.random() * 100) + '%',
                '--random-y':(Math.random() * 100) + '%'
              })
            }
            setTimeout(() => {
              if(this.mainDisplay!='buttom'&&config.setting().config.useAnimeBackground){
                this.random()
              } else {
                this.dynFunctionRunning =false
              }
            }, 6*1000);
          }
        },
        props: {
            imgSrc: String,
            mainDisplay: String,
            // colorData: Object,
            dynamic: Boolean
        },
        watch: {
            mainDisplay:{
              handler: function (newVal,oldVal) {
                    if(this.dynFunctionRunning == false&&newVal!='buttom'&&config.setting().config.useAnimeBackground){
                      this.$nextTick(()=>{
                        this.random()
                      })
                    }
                },
                deep: true
            }
        }
        
    }
    

</script>

<template>
  <!-- :style="{background: (colorData)?colorData[0].color:null}"  -->
    <div  v-if="(mainDisplay != 'buttom')" 
    
     v-bind:class="['player-background',(mainDisplay,config.setting().config.useAnimeBackground == true)?'dyn':'']">
      <!-- {{ colorData }} -->
      <div v-for="n in 4" ref="block" :style="{
        backgroundImage: 'url(' + imgSrc + '?param=128y128'+')',
        ...position[n]
      }">
        
    </div>
      </div>
</template>

<style>
  .player-background{
    animation: spawnplayerbackground 0.3s linear;
  }
  @keyframes spawnplayerbackground {
    from{
      opacity: 0;
    }
    to{
      opacity: 1;
    }
  }
</style>