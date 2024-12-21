<template>
    <div class="relativeBox">
        <!-- 浮动盒子的容器，根据主题和可见性动态应用样式 -->
        <div ref="floatingBox"
            :class="['floating-box', `theme-${theme}`, { 'visible': isVisible, 'not-visible': !isVisible }]"
            :style="floatingBoxStyle">
            <!-- 插槽，用于插入悬浮内容 -->
            <slot name="suspendContent"></slot>
        </div>
                <!-- 占位符容器，用于鼠标悬停时触发悬浮盒子的显示 -->
                <div ref="placeholder" @mouseenter="handleMouseEnter" @mouseleave="handleMouseLeave">
            <!-- 插槽，用于插入占位符内容 -->
            <slot name="placeholder"></slot>
        </div>
    </div>

</template>

<script>
import { ref, computed, onMounted, watch, nextTick } from 'vue';

export default {
    name: 'FloatingBox', // 组件名称
    props: {
        // 悬浮盒子的方向，默认为'right'
        direction: {
            type: String,
            default: 'right',
        },
        // 是否仅在鼠标悬停时显示悬浮盒子，默认为false
        hoverOnly: {
            type: Boolean,
            default: false,
        },
        // 悬浮盒子的可见性，默认为true
        visibility: {
            type: Boolean,
            default: true,
        },
        // 悬浮盒子的原点坐标，默认为[0, 0]
        originPoint: {
            type: Array,
            default: () => [0, 0],
        },
        // 悬浮盒子的主题，默认为'black'
        theme: {
            type: String,
            default: 'black',
        },
        // 悬浮盒子的最大宽度，默认为'fit-content'
        maxWidth: {
            type: String,
            default: 'fit-content',
        },
    },
    setup(props) {
        const floatingBox = ref(null); // 引用悬浮盒子的DOM元素
        const placeholder = ref(null); // 引用占位符的DOM元素
        const isVisible = ref(props.visibility && !props.hoverOnly); // 控制悬浮盒子的可见性
        let timeoutId = null; // 用于延时显示悬浮盒子的计时器

        // 计算悬浮盒子的样式
        const floatingBoxStyle = computed(() => {
            const style = {
                maxWidth: props.maxWidth,
                // width: '',
                visibility: isVisible.value ? 'visible' : 'hidden',
            };
            // 根据方向和原点坐标计算悬浮盒子的位置
            const [originX, originY] = props.originPoint;
            const placeholderRect = placeholder.value?.getBoundingClientRect();
            const floatingBoxRect = floatingBox.value?.getBoundingClientRect();

            if (placeholderRect && floatingBoxRect) {
                const directions = [props.direction, 'top', 'right', 'bottom', 'left', 'center'];
                const dirToVal = {
                    top: 'height',
                    bottom: 'height',
                    left: 'width',
                    right: 'width'
                };
                const reverse = {
                    top: 'bottom',
                    bottom: 'top',
                    left: 'right',
                    right: 'left'
                };
                let positionFound = false;
                for (let dir of directions) {
                    if (positionFound) break;

                    const isHorizontal = dir === 'left' || dir === 'right';
                    const isVertical = dir === 'top' || dir === 'bottom';
                    console.log(floatingBoxRect);
                    if (dir === 'center') {
                        style.top = `${originY}px`;
                        style.left = `${originX}px`;
                    } else if (isHorizontal == true) {
                        style[reverse[dir]] = `${placeholderRect[dirToVal[dir]] + 5}px`;
                        style.top = `${ placeholderRect.height / 2 - floatingBoxRect.height / 2}px`;
                    } else if (isVertical == true) {
                        style.top = `${placeholderRect[dirToVal[dir]]} + 5}px`;
                        style.left = `${placeholderRect.left + placeholderRect.width / 2 - floatingBoxRect.width / 2}px`;
                    }

                    // 检查悬浮盒子是否在视口内，并调整位置
                    // const boxLeft = parseFloat(style.left);
                    // const boxTop = parseFloat(style.top);
                    // const boxRight = boxLeft + floatingBoxRect.width;
                    // const boxBottom = boxTop + floatingBoxRect.height;

                    // if (boxLeft >= 0 && boxRight <= window.innerWidth && boxTop >= 0 && boxBottom <= window.innerHeight) {
                    //     positionFound = true;
                    // } else {
                    //     // 如果不在视口内，重置样式
                    //     style.top = '';
                    //     style.left = '';
                    //     style[dir] = '';
                    // }
                    positionFound = true;

                }
            }

            return style;
        });

        // 检查并调整悬浮盒子的位置
        const checkAndAdjustPosition = () => {
            const floatingBoxRect = floatingBox.value?.getBoundingClientRect();
            if (floatingBoxRect) {
                const boxLeft = parseFloat(floatingBoxStyle.value.left);
                const boxTop = parseFloat(floatingBoxStyle.value.top);
                const boxRight = boxLeft + floatingBoxRect.width;
                const boxBottom = boxTop + floatingBoxRect.height;

                if (boxRight > window.innerWidth) {
                    floatingBoxStyle.value.left = `${window.innerWidth - floatingBoxRect.width}px`;
                }
                if (boxBottom > window.innerHeight) {
                    floatingBoxStyle.value.top = `${window.innerHeight - floatingBoxRect.height}px`;
                }
                if (boxLeft < 0) {
                    floatingBoxStyle.value.left = '0px';
                }
                if (boxTop < 0) {
                    floatingBoxStyle.value.top = '0px';
                }
            }
        };

        // 鼠标进入占位符时的处理函数
        const handleMouseEnter = () => {
            if (props.hoverOnly && props.visibility) {
                timeoutId = setTimeout(() => {
                    isVisible.value = true;
                }, 500);
            }
        };

        // 鼠标离开占位符时的处理函数
        const handleMouseLeave = () => {
            if (props.hoverOnly) {
                clearTimeout(timeoutId);
                isVisible.value = false;
            }
        };

        onMounted(() => {
            checkAndAdjustPosition();
            // 监听方向、原点坐标和可见性的变化，并调整位置和可见性
            watch(() => [props.direction, props.originPoint, props.visibility], () => {
                if (!props.hoverOnly || (props.hoverOnly && props.visibility)) {
                    checkAndAdjustPosition();
                }
                isVisible.value = props.visibility && !props.hoverOnly;
            });
        });

        return {
            floatingBox,
            placeholder,
            isVisible,
            floatingBoxStyle,
            handleMouseEnter,
            handleMouseLeave,
        };
    },
};
</script>

<style scoped>
.relativeBox{
    position: relative;
    width: fit-content;
    height: fit-content;
}
.floating-box {
    position: absolute;
    min-width: 100px !important;
    /* 悬浮盒子的默认样式 */
    opacity: 0;
    padding: .4em .7em;
    border-radius: 0.64em;
    transition: 0.3s;
    box-shadow: var(--Shadow-value-normal);
}

.theme-light {
    /* 浅色主题样式 */
    background-color: #fff;
    color: #333;
}

.theme-black {
    /* 黑色主题样式 */
    background-color: #333;
    color: #fff;
}

.visible {
    visibility: visible;
    opacity: 1;

}

.not-visible {
    visibility: hidden;
    opacity: 0;

}

</style>