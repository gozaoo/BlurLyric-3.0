<script>
import contextMenu from './base/contextMenu.vue';

import base from '../js/base';
export default{
    data(){
        return{

        }
    },
    components:{
        contextMenu
    },
    methods:{
        copy:base.copy
    },
    mounted(){
        console
        
    }
}
</script>
<template>
    <contextMenu :menuItems="[
        { iconClass:['bi','bi-clipboard'] , name: '复制文字', handleClick: () => { copy(this.$refs.innerDom.innerText) } },
      ]
    ">
    <div ref="innerDom">
        <slot/>
    </div>
        
    </contextMenu>
</template>