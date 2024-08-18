export default {
    create(element,onClickCallBack, onMoveCallBack, moveEndCallBack) {
        let info = {
            screenX: null,
            screenY: null,
            originScreenX: null,
            originScreenY: null,

            offsetX: null,
            offsetY: null,

            speedX: 0,
            speedY: 0,

            speedDirectionX: 'none',
            speedDirectionY: 'none',
            offsetDirectionX: 'none',
            offsetDirectionY: 'none',

            lastScreenX: null,
            lastScreenY: null,
            lastTime: null

        };

        let addListen = () => {
            element.addEventListener('mousedown', onMouseDown);
            element.addEventListener('touchstart', onTouchStart);
        };

        let destroy = () => {
            element.removeEventListener('mousedown', onMouseDown);
            element.removeEventListener('touchstart', onTouchStart);
            document.removeEventListener('mousemove', onMouseMove);
            document.removeEventListener('mouseup', onMouseUp);
            document.removeEventListener('touchmove', onTouchMove);
            document.removeEventListener('touchend', onTouchEnd);
            document.removeEventListener('touchcancel', onTouchEnd);
        };
        
        let onMove = (event) => {
            // 更新info对象的值
            // 例如：info.screenX = event.screenX;
            // 你需要根据实际情况更新这些值
            onMoveCallBack(info);
        };

        let onDragEnd = () => {
            // 在这里处理拖拽结束的逻辑
            moveEndCallBack(info);

            // 移除mousemove和mouseup事件监听器
            document.removeEventListener('mousemove', onMouseMove);
            document.removeEventListener('mouseup', onMouseUp);
        };

        // 鼠标事件处理函数
        let onMouseDown = (event) => {
            // 记录初始位置
            info.originScreenX = event.screenX;
            info.originScreenY = event.screenY;
            info.screenX = event.screenX;
            info.screenY = event.screenY;

            // 添加mousemove和mouseup事件监听器到document
            onClickCallBack(info)
            document.addEventListener('mousemove', onMouseMove);
            document.addEventListener('mouseup', onMouseUp);
        };

        let onMouseMove = (event) => {
            event.preventDefault()

            if(event.buttons != 1){
                onMouseUp()
                return
            }

            updateSpeedAndDirection(event.screenX, event.screenY, event.timeStamp);
            // 计算移动距离
            info.offsetX = event.screenX - info.originScreenX;
            info.offsetY = event.screenY - info.originScreenY;
            info.screenX = event.screenX;
            info.screenY = event.screenY;
            // 更新方向和速度
            // 你需要根据实际情况更新这些值

            onMove(event);
        };

        let onMouseUp = (event) => {
            onDragEnd();
        };

        // 触摸事件处理函数
        let onTouchStart = (event) => {
            // 记录初始位置
            let touch = event.touches[0] || event.changedTouches[0];
            info.originScreenX = touch.screenX;
            info.originScreenY = touch.screenY;
            info.screenX = touch.screenX;
            info.screenY = touch.screenY;
            // 添加touchmove和touchend事件监听器到element
            onClickCallBack(info)
            document.addEventListener('touchmove', onTouchMove);
            document.addEventListener('touchend', onTouchEnd);
            document.addEventListener('touchcancel', onTouchEnd);
        };

        let onTouchMove = (event) => {

            let touch = event.touches[0] || event.changedTouches[0];
            updateSpeedAndDirection(touch.screenX, touch.screenY, event.timeStamp);

            // 计算移动距离
            info.offsetX = touch.screenX - info.originScreenX;
            info.offsetY = touch.screenY - info.originScreenY;
            info.screenX = touch.screenX;
            info.screenY = touch.screenY;
            // 更新方向和速度
            // 你需要根据实际情况更新这些值

            onMove(event);
        };

        let onTouchEnd = (event) => {
            // 移除touchmove和touchend事件监听器
            document.removeEventListener('touchmove', onTouchMove);
            document.removeEventListener('touchend', onTouchEnd);
            document.removeEventListener('touchcancel', onTouchEnd);

            onDragEnd();
        };

        // 更新速度和方向的辅助函数
        let updateSpeedAndDirection = (currentX, currentY, currentTime) => {
            if (info.lastTime && info.lastScreenX !== null && info.lastScreenY !== null) {
                let deltaTime = currentTime - info.lastTime;
                let deltaX = currentX - info.lastScreenX;
                let deltaY = currentY - info.lastScreenY;
                let speedX = deltaX / deltaTime || 0;
                let speedY = deltaY / deltaTime || 0;

                info.speedX = speedX;
                info.speedY = speedY;

                info.speedDirectionX = deltaX > 0 ? 'right' : deltaX < 0 ? 'left' : 'none';
                info.speedDirectionY = deltaY > 0 ? 'down' : deltaY < 0 ? 'up' : 'none';

                info.offsetDirectionX = deltaX > 0 ? 'right' : deltaX < 0 ? 'left' : 'none';
                info.offsetDirectionY = deltaY > 0 ? 'up' : deltaY < 0 ? 'down' : 'none';
            }

            info.lastScreenX = currentX;
            info.lastScreenY = currentY;
            info.lastTime = currentTime;
        };

        addListen(); // 初始化监听器
        return {
            destroy
        };
    }
};
