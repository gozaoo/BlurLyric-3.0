let timeout;
export default {
    // element: 要监听的元素
    // fn: 执行的函数，传入true或false
    addOnhoverListener(element, fn) {
        // 添加桌面端的鼠标事件
        element.addEventListener('mouseenter', () => fn(true));
        element.addEventListener('mouseleave', () => fn(false));

        // 添加移动端的触摸事件
        element.addEventListener('touchstart', (e) => {
            // 防止触摸时触发点击事件
            fn(true);
        }, {
            passive: false
        });
        element.addEventListener('touchend', (e) => {
            // 防止触摸时触发点击事件
            fn(false);
        }, {
            passive: false
        });
    },
    removeOnhoverListener(element, fn) {
        // 移除桌面端的鼠标事件
        element.removeEventListener('mouseenter', () => fn(true));
        element.removeEventListener('mouseleave', () => fn(false));

        // 移除移动端的触摸事件
        element.removeEventListener('touchstart', (e) => {
            fn(true);
        }, {
            passive: false
        });
        element.removeEventListener('touchend', (e) => {
            fn(false);
        }, {
            passive: false
        });
    },
    addSlideEvent(element, fn, config) {
        let startX, startY, startTime, lastDirection;
        let slideTimestamp;
        let lastMoveTime, lastMoveX, lastMoveY = null;

        const handleMouseDown = (e) => {
            // e.preventDefault();
            startX = e.clientX || e.touches[0].clientX;
            startY = e.clientY || e.touches[0].clientY;
            startTime = Date.now();
            lastDirection = null;
            slideTimestamp = null;
            document.addEventListener('mousemove', handleMouseMove);
            document.addEventListener('mouseup', handleMouseUp);
            document.addEventListener('touchmove', handleMouseMove);
            document.addEventListener('touchend', handleMouseUp);
        };
        const handleMouseMove = (e) => {
            const currentTime = Date.now();
            const currentX = e.clientX || e.touches[0].clientX;
            const currentY = e.clientY || e.touches[0].clientY;
            const moveX = currentX - startX;
            const moveY = currentY - startY;
            const direction = getDirection(moveX, moveY);
            if (lastMoveTime !== null) {
                const timeInterval = currentTime - lastMoveTime; // 时间间隔，单位是毫秒
                const displacementX = currentX - lastMoveX; // 水平位移
                const displacementY = currentY - lastMoveY; // 垂直位移
                // 计算瞬时速度，单位是像素每0.1秒
                const speedX = displacementX / (timeInterval / 100);
                const speedY = displacementY / (timeInterval / 100);
                const speed = Math.sqrt(speedX * speedX + speedY * speedY); // 合速度，不考虑方向
                if (direction !== lastDirection && lastMoveTime !=null) {
                    lastDirection = direction;
                    fn({
                        type: 'slide',
                        direction,
                        moveX,
                        moveY,
                        slideTimestamp:currentTime,
                        speed
                    });
                    cleanup();
                    // console.log(speed);
                }
            }

        lastMoveTime = currentTime;
        lastMoveX = currentX;
        lastMoveY = currentY;

        };

        const getDirection = (moveX, moveY) => {
            // 只返回单一方向
            if (Math.abs(moveX) > Math.abs(moveY)) {
                return moveX > 0 ? 'right' : 'left';
            } else if (Math.abs(moveY) > Math.abs(moveX)) {
                return moveY > 0 ? 'down' : 'up';
            }
            return null;
        };
        const handleMouseUp = (e) => {
            if (lastDirection === null) {
                fn({
                    type: 'click'
                });
            } else {
                handleMouseMove(e)
            }
            cleanup();
        };
        const cleanup = () => {
            startX = null;
            startY = null;
            lastDirection = null;
            slideTimestamp = null;
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
            document.removeEventListener('touchmove', handleMouseMove);
            document.removeEventListener('touchend', handleMouseUp);
        };
        element.addEventListener('mousedown', handleMouseDown);
        element.addEventListener('touchstart', handleMouseDown);
        return {
            removeSlideEvent() {
                element.removeEventListener('mousedown', handleMouseDown);
                element.removeEventListener('touchstart', handleMouseDown);
            }
        }
    },
    
    onWindowsResize(fn,times) {
        let retunFunction = () => {
            console.log(timeout);;
            requestAnimationFrame(()=>{
                timeout = setTimeout(() => {
                    fn();
                }, times||100);

            })
        }
        window.addEventListener('resize', retunFunction);
    return {removeWindowsResize: ()=>window.removeEventListener('resize',retunFunction)}

    }

}