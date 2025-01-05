<!-- MessageDisplay.vue -->
<template>
    <div class="messageListBox">
        <div :class="['global_backgroundblur_light',message.type,message.state]" v-for="message in messageList" :key="message.id">
            <div class="Alert" v-if="message.type == 'Alert'">
                <!-- 显示 Alert 类型的消息 -->
                <span>{{ message.content }}</span>
            </div>
            <div :class="['LongMessage',message.state]"  v-if="message.type == 'LongMessage'">
                <!-- 显示 Message 类型的消息 -->
                <span>{{ message.content }}</span>
            </div>
            <div class="Message" v-else-if="message.type == 'Message'">
                <!-- 显示 Message 类型的消息 -->
                <span>{{ message.content }}</span>
            </div>

        </div>

    </div>

</template>

<script>

export default {
    name: 'MessageDisplay',
    inject: ['messageList']
};
</script>

<style scoped>
.messageListBox {
    position: absolute;
    left: 0;
    top: 0;
    right: 0;
    bottom: 115px;
    /* background-color: #0001; */
    z-index: 201;
    pointer-events: none;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 8px;
}

.messageListBox>* {
    /* background: #; */
    font-size: .8rem;
    padding: 9px 14px;
    width: fit-content;
    box-shadow: var(--Shadow-value-normal);
    border-radius: 11px;
    min-height: fit-content;
    margin: 0 auto;
    position: relative;
    opacity: 0;
    box-sizing: content-box;
    transition: 0.26s;
}

.messageListBox> .LongMessage.display{
    opacity: 1;
}
.message {
    overflow: hidden;
    animation: in 6.5s ease-in-out;
}

@keyframes in {
    0% {
        opacity: 0;
    }

    4% {
        opacity: 1;
    }

    96% {
        opacity: 1;
    }

    100% {
        opacity: 0;
    }
}
</style>