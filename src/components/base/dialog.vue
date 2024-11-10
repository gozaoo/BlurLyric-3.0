<script>

    export default {
        data(){
            return {}
        },
        props: {
            cancel: Function,
            finish: Function,

        },
        components:{
        }
    }
</script>
<template>
    <div class="background">
        <div class="container">
            <slot  />
            <!-- <button>a</button>
              -->
            <iconFlexRow class="toRightEnd">
                <iconWithText @click="cancel" type="background">
                    <template #icon><i class="bi bi-x"></i></template>
                    <template #text>取消</template>
                </iconWithText>
                <iconWithText @click="finish" type="buttom_active">
                    <template #icon><i class="bi bi-check"></i></template>
                    <template #text>确定</template>
                </iconWithText>
            </iconFlexRow>
        </div>
    </div>
</template>
<style>
    .background{
        position: fixed;
        left: 0;
        top: 0;
        right: 0;
        bottom: 0;
        background-color: #0001;
        z-index: 200;
    }
    .toRightEnd{
        width: fit-content;
        
        margin: 11px 0 0 auto;

    }
    .container{
        background: #fff;
		padding: 14px;
        min-width: min(240px,80vw);
        min-height: min(100px,60vh);
        width: fit-content;
        box-shadow: var(--Shadow-value-card-high);
		border-radius: 11px;
        margin: 20vh auto;
        position: relative;
        box-sizing:content-box;
        overflow: hidden;
    }
</style>