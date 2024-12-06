<script>
import manager from '../../api/manager';

  manager
    export default{
        data(){
            return{
                anime: undefined,
                position:[
                  {},
                  {},
                  {},
                  {}
                ],
                dynFunctionRunning:false,
                imgSrc: null,
                objectURL: null
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
          //   setTimeout(() => {

          //   if(this.dynFunctionRunning ==false&&this.musicInfoPagePosition!='bottom'){
          //         this.random()
          //   }
          // },3000)

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
            // setTimeout(() => {
            //   if(this.musicInfoPagePosition!='bottom'&&this.dynamic == true){
            //     this.random()
            //   } else {
            //     this.dynFunctionRunning =false
            //   }
            // }, 6*1000);
          },
          async fetchURL(){
            if (this.objectURL) {
                  URL.revokeObjectURL(this.objectURL); // 销毁之前的ObjectURL
                }
            const newSrc = await manager.tauri.getAlbumCover(this.coverId);
            this.objectURL = newSrc; // 更新ObjectURL
            this.imgSrc = newSrc;
          }
        },
        mounted(){
          this.fetchURL()
        },
        unmounted(){
          if (this.objectURL) {
                  URL.revokeObjectURL(this.objectURL); // 销毁之前的ObjectURL
                }
        },
        props: {
          coverId: Number,
            musicInfoPagePosition: String,
            // colorData: Object,
            dynamic: Boolean
        },
        watch: {
          musicInfoPagePosition:{
              handler: function (newVal,oldVal) {
                    if(this.dynFunctionRunning == false&&newVal!='bottom'){
                      this.$nextTick(()=>{
                        this.random()
                      })
                    }
                },
                deep: true
            },
            coverId:{
              handler: function(newid,oldid){
                this.fetchURL()
              }
            }
        }
        
    }
    

</script>

<template>
  <!-- :style="{background: (colorData)?colorData[0].color:null}"  -->
    <div  v-if="(musicInfoPagePosition != 'bottom')" 
    
     v-bind:class="['player-background',(dynamic)?'dyn':'']">
      <!-- {{ colorData }} --> 
      <div v-for="n in 4" ref="block" :style="{
        backgroundImage: 'url(' + this.imgSrc + ')',
        ...position[n]
      }">
        
    </div>
      </div>
</template>

<style>
  .player-background{
    animation: spawnplayerbackground 0.3s linear;
  filter: blur(8vmin) saturate(200%) contrast(40%) brightness(90%) ;
  transition: background .4s;
  margin: -10vh 0 0 -10vw;

  }
  @keyframes spawnplayerbackground {
    from{
      opacity: 0;
    }
    to{
      opacity: 1;
    }
  }
  
.player-background>div {
  position: absolute;
  width: 60vw;
  height: 60vh;
  /* background-color: var(--color1); */
  z-index: -2;
  background-size: 200%;
  /* opacity: 1; */
  background-repeat: no-repeat;
  /* background-size: cover; */
  /* will-change: filter,background; */
    will-change: background;
    transition: background .4s;
}


.player-background>div{
  --animationName: background1;
  transition: background .4s,background-position 6s ease-in-out;
  background-position: var(--random-x) var(--random-y);
  /* transform: rotate(calc(calc(var(--random-x) / 100% ) * 180deg)); */

}

.player-background.dyn>div{
  transition: background .4s,background-position 6s ease-in-out;

}

.player-background>div:nth-child(1){
  left: 60vw;
}
.player-background>div:nth-child(2){
  top: 60vh;
}.player-background>div:nth-child(3){
  top: 60vh;
  left: 60vw;
}
</style>