<template>
    <div  @mouseover="showMenu" @mouseleave="hideMenu" class="context-menu-container">
        <div class="slot">
            <slot></slot>
        </div>
        <div v-if="visible" class="vue-context-menu">
            <div :style="{
                maxHeight: '32px'
            }"  v-for="(item, index) in menuItems" :key="index" @click="item.handleClick">
                <div v-if="item.name=='&Split Lines'" class="hr"></div>
                <div class="li" v-if="item.name!='&Split Lines'">
                    <div v-if="item.iconClass" :class="[...item.iconClass,'icon']"></div>
                    {{ item.name }}
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    export default {
        name: 'HoverContextMenu',
        data() {
            return {
                visible: false,
                // position: {
                    // x: 0,
                    // y: 0
                // },
            };
        },
        props: {
            menuItems: Array,
            debug_alwaysOpen: Boolean,
        },
        methods: {
            showMenu(event) {
                // this.position.x = event.clientX - 67;
                // this.position.y = event.clientY + 4; // 修改为clientY
                this.visible = true;
            },
            hideMenu() {
                if (!this.debug_alwaysOpen) {
                    this.visible = false;
                }
            },
            // 移除与右键菜单相关的全局事件监听器
            addGlobalListeners() {},
            removeGlobalListeners() {}
        },
        mounted() {
            if (this.debug_alwaysOpen) {
                this.visible = true;
            }
        },
        beforeDestroy() {
            // 由于不再使用全局事件监听器，这里不再需要移除
        }
    };
</script>


<style scoped>
    .context-menu-container {
        position: relative;
        user-select: none;
    }

    .icon {
        float: left;
        /* width: 25px; */
        padding: 0 12px 0 4px;
        box-sizing: border-box;
    }

    /* 样式保持不变 */
    .vue-context-menu {
        position: absolute;
        left: 100%;
        top: 50%;
        border-radius: 9px;
        max-width: 200px;
        background-color: #fff;
        box-sizing: border-box;
        padding: 6px;
        min-width: max-content;
        display: flex;
        flex-direction: column;
        font-size: 15px;
        transition: 0.3s cubic-bezier(.3, .7, .2, 1);
        gap: 5px;
        user-select: none;
        border: 1px solid #0002;
        transform:  scale(1) translateY(-50%);
        box-shadow: 0 0 15px rgba(0, 0, 0, 0.1);
        transform-origin: 0% 0%;
        animation: spawn_context_menu 0.2s cubic-bezier(.3, .7, .2, 1)
    }

    .vue-context-menu div.hr {
        background: #0002;
        width: calc(100% - 12px);
        margin: 0 6px;
        height: 1px;
    }

    .vue-context-menu div.li {
        padding: 6px 10px 6px 6px;
        border-radius: 6px;
        cursor: pointer;
        transition: 0.314s cubic-bezier(.3, .7, .2, 1);
        /* margin-top: auto; */
        width: 100%;
        display: inline-block;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        box-sizing: border-box;

    }

    .li_hoverMenu{
        padding: 6px 25px 6px 6px;
        border-radius: 6px;
        cursor: pointer;
        transition: 0.314s cubic-bezier(.3, .7, .2, 1);

    }
    
    .flexRight{
        position: absolute;
        right: 0;
        color: #00000080;
    }
    .vue-context-menu div.li:hover,.li_hoverMenu:hover {
        background-color: #00000009;
    }
    @keyframes spawn_context_menu {
        from {
            transform: scale(.7) translate( -10px ,calc(-50% - 10px));
            opacity: 0;
        }

        to {
            transform: scale(1)  translate(0,-50%);
            opacity: 1;


        }
    }
</style>