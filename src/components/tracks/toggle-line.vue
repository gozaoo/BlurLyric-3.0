<script>
export default {
    data() {
        return {
            value: false
        }
    },
    props: {
        modelValue: Boolean, // 使用 modelValue 作为 v-model 的绑定值
        type: {
            type: String,
            default: 'normal' // 设置默认值为 'normal'
        },
    },
    emits: ['update:modelValue'], // 声明组件会发出的事件
    methods: {
    },
    // computed:{
    // value(){return this.modelValue}
    // },
    created() {
        this.value = this.modelValue
    },
    watch: {
        value: {
            handler() {
                this.$emit('update:modelValue', this.value); // 使用 update:modelValue 代替 changeState

            }
        }
    }
}
</script>
<template>
    <div class="row">
        <slot name="icon" />

        <div>
            <div class="name">
                <slot name="text" />
            </div>
        </div>
        <toggle :type="type" v-model="value" class="flexRight"></toggle>
    </div>
</template>
<style scoped>
.row {
    /* background-color: #0001; */
    height: 32px;
    padding: 6px 12px;
    display: flex;
    user-select: none;
    align-items: center;
    gap: 6px;
    width: 100%;
    overflow: hidden;

}

i {
    font-size: 20px;
    height: 25px;
    width: 25px;
}

.name {
    font-size: small;
}

.path {
    font-size: small;
}

.flexRight {
    margin-left: auto
}
</style>