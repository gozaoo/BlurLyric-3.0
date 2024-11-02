<script>
// import contextMenu from './base/contextMenu.vue';
// import baseMethods from '../js/baseMethods';

export default{
    data(){
        return{
            
        }
    },
    components:{
        contextMenu
    },
    methods:{
        copy:baseMethods.copy
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