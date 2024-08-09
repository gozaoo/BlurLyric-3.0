<template>
    <div class="context-menu-container" @click="hideMenu">
        <div @contextmenu="handleContextMenu" class="slot">
            <slot></slot>
        </div>
        <div v-if="visible" :style="{ top: position.y + 'px', left: position.x + 'px' }" class="vue-context-menu">
            <div :style="{
                maxHeight: '32px'
            }" v-for="(item, index) in menuItems" :key="index" @click="item.handleClick">
                <div v-if="item.type=='hr'" class="hr"></div>
                <hoverMenu  class="li_hoverMenu"  :menuItems="item.menuItems" v-if="item.type=='hoverMenu'" >
                    <div v-if="item.iconClass" :class="[...item.iconClass,'icon']"></div>

                        {{ item.name }}
                        <i class="bi bi-chevron-right flexRight"></i>

                </hoverMenu>
                <div  v-if="item.type==undefined || item.type=='normal'" class="li" >
                    <div v-if="item.iconClass" :class="[...item.iconClass,'icon']"></div>
                        {{ item.name }}
                </div>

            </div>
        </div>
    </div>
</template>

<script>
import hoverMenu from './hoverMenu.vue';
    export default {
        name: 'VueContextMenu',
        components:{
            hoverMenu

        },
        data() {
            return {
                visible: false,
                position: {
                    x: 0,
                    y: 0
                },
            };
        },
        props: {
            menuItems: Array,
            debug_alwaysOpen: Boolean,
        },
        methods: {
            showMenu(event) {
                this.position.x = event.clientX - 67;
                this.position.y = event.offsetY + 4;
                this.visible = true;
                this.addGlobalListeners();
                event.preventDefault()
                event.stopPropagation();
            },
            hideMenu() {
                if (!this.debug_alwaysOpen) {
                    this.visible = false;
                    this.removeGlobalListeners();
                }
            },
            handleContextMenu(event) {
                this.showMenu(event);
            },
            shouldShowMenu(event) {
                return true;
            },
            addGlobalListeners() {
                document.addEventListener('click', this.hideMenu);
                document.addEventListener('scroll', this.hideMenu);
                document.addEventListener('keydown', this.hideMenu);
                document.addEventListener('contextmenu', this.hideMenu);
            },
            removeGlobalListeners() {
                document.removeEventListener('click', this.hideMenu);
                document.removeEventListener('scroll', this.hideMenu);
                document.removeEventListener('keydown', this.hideMenu);
                document.removeEventListener('contextmenu', this.hideMenu);
            }
        },
        mounted() {
            if (this.debug_alwaysOpen) {
                this.visible = true;
            }
        },
        beforeDestroy() {
            this.removeGlobalListeners();
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
        z-index: 9999;
        border-radius: 9px;
        max-width: 200px;
        background-color: #fffa;
        box-sizing: border-box;
        padding: 6px;
        display: flex;
        flex-direction: column;
        font-size: 15px;
        backdrop-filter: blur(12px);
        transition: 0.3s cubic-bezier(.3, .7, .2, 1);
        gap: 5px;
        width: max-content;
        user-select: none;
        border: 1px solid #0002;
        box-shadow: 0 0 15px rgba(0, 0, 0, 0.1);
        transform-origin: 0% 0%;
        animation: spawn_context_menu 0.4s cubic-bezier(.3, .7, .2, 1)
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
        width: 100%;
        -webkit-box-orient: vertical;
        -webkit-line-clamp: 1;
        display: inline-block;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        box-sizing: border-box;

    }

    .li_hoverMenu{
        padding: 6px 32px 6px 6px;
        border-radius: 6px;
        cursor: pointer;
        transition: 0.314s cubic-bezier(.3, .7, .2, 1);

    }
    
    .flexRight{
        position: absolute;
        right: 6px;
        color: #00000080;
    }
    .vue-context-menu div.li:hover,.li_hoverMenu:hover {
        background-color: #00000009;
    }
    @keyframes spawn_context_menu {
        from {
            transform: scale(.5) translate(-10px, -10px);
            opacity: 0;
        }

        to {
            transform: none;
            opacity: 1;


        }
    }
</style>