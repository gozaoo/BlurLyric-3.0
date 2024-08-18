<template>
    <i :class="_class"></i>
</template>
<script>
    export default{
        inject:['trackState'],
        data(){
            return {
            }
        },
        computed:{
            _class(){
                switch (this.trackState.playMode) {
                    case 'loopPlaylist':
                        return ['bi','bi-repeat']
                    case 'loopSingle':
                        return ['bi','bi-repeat-1']
                    case 'stopAfterSingle':
                        return ['bi','bi-sign-stop']
                    case 'randomPlay':
                        return ['bi','bi-shuffle']
                    case 'smartRecommend':
                        return ['bi','bi-lightbulb']
                    default:
                        return ['bi', 'bi-question-circle-fill'];
                }
            }
        }
    }
</script>