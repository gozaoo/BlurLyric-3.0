export default {
    debounce(func, wait) {
        let timeoutId = null;

        return function (...args) {
            if (timeoutId) {
                clearTimeout(timeoutId);
            }

            timeoutId = setTimeout(() => {
                func.apply(this, args);
            }, wait);
        };
    },
    copy(text) {
        const textarea = document.createElement('textarea');
        textarea.value = text;
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);
    },
    isPossibleLocalPath(str) {
        // 检查str是否为字符串
        if (typeof str !== 'string') {
            return false;
        }

        // 检查是否以Windows风格的路径开始（例如C:\或D:\）
        const isWindowsPath = /^[A-Za-z]:\\/.test(str);

        // 检查是否以Unix/Linux风格的路径开始（例如/)
        const isUnixPath = /^\//.test(str);

        // 如果是其中之一，则返回true，否则返回false
        return isWindowsPath || isUnixPath;
    },
    progressBarReg(progressBarDom, getCurrentProgress, progressUpdate) {
        let info = {
            currentProgress: 0,
            beforeDragProgress: 0,
            dragProgress: 0,
            BeforeHoveringProgress: 0,
            hoveringProgress: 0,
            draging: false,
            hovering: false,
            domWidth: null,
            offsetX: null,
        };

        let onInfoChange = () => {
            progressUpdate(info);
            progressBarDom.style.setProperty('--currentProgress', info.currentProgress);
            progressBarDom.style.setProperty('--beforeDragProgress', info.beforeDragProgress);
            progressBarDom.style.setProperty('--dragProgress', info.dragProgress);
            progressBarDom.style.setProperty('--BeforeHoveringProgress', info.BeforeHoveringProgress);
            progressBarDom.style.setProperty('--hoveringProgress', info.hoveringProgress);
            progressBarDom.style.setProperty('--draging', info.draging);
            progressBarDom.style.setProperty('--hovering', info.hovering);
        };

        const handleTouchStart = (e) => {
            if (e.touches.length === 1) {
                info.draging = true;
                info.beforeDragProgress = info.currentProgress;
                const rect = progressBarDom.getBoundingClientRect();
                info.offsetX = e.touches[0].clientX - rect.left;
                info.domWidth = rect.width;
                onInfoChange();
            }
        };

        const handleTouchMove = (e) => {
            if (info.draging && e.touches.length === 1) {
                const x = e.touches[0].clientX;
                const rect = progressBarDom.getBoundingClientRect();
                info.dragProgress = Math.max(0, Math.min(1, (x - rect.left - info.offsetX) / info.domWidth));
                info.currentProgress = info.dragProgress;
                onInfoChange();
            }
        };

        const handleTouchEnd = () => {
            if (info.draging) {
                info.draging = false;
                info.currentProgress = info.dragProgress;
                onInfoChange();
            }
        };

        const handleMouseDown = (e) => {
            info.draging = true;
            info.beforeDragProgress = info.currentProgress;
            const rect = progressBarDom.getBoundingClientRect();
            info.offsetX = e.clientX - rect.left;
            info.domWidth = rect.width;
            onInfoChange();
        };

        const handleMouseMove = (e) => {
            if (info.draging) {
                const x = e.clientX;
                const rect = progressBarDom.getBoundingClientRect();
                info.dragProgress = Math.max(0, Math.min(1, (x - rect.left - info.offsetX) / info.domWidth));
                info.currentProgress = info.dragProgress;
                onInfoChange();
            }
        };

        const handleMouseUp = (e) => {
            if (info.draging) {
                info.draging = false;
                info.currentProgress = info.dragProgress;
                onInfoChange();
            }
        };

        const handleMouseEnter = (e) => {
            info.hovering = true;
            info.BeforeHoveringProgress = info.currentProgress;
            onInfoChange();
        };

        const handleMouseLeave = (e) => {
            info.hovering = false;
            info.currentProgress = info.BeforeHoveringProgress;
            onInfoChange();
        };

        progressBarDom.addEventListener('touchstart', handleTouchStart);
        progressBarDom.addEventListener('touchmove', handleTouchMove);
        progressBarDom.addEventListener('touchend', handleTouchEnd);
        progressBarDom.addEventListener('mousedown', handleMouseDown);
        progressBarDom.addEventListener('mousemove', handleMouseMove);
        progressBarDom.addEventListener('mouseup', handleMouseUp);
        progressBarDom.addEventListener('mouseenter', handleMouseEnter);
        progressBarDom.addEventListener('mouseleave', handleMouseLeave);

        return {
            cancelReg: () => {
                progressBarDom.removeEventListener('touchstart', handleTouchStart);
                progressBarDom.removeEventListener('touchmove', handleTouchMove);
                progressBarDom.removeEventListener('touchend', handleTouchEnd);
                progressBarDom.removeEventListener('mousedown', handleMouseDown);
                progressBarDom.removeEventListener('mousemove', handleMouseMove);
                progressBarDom.removeEventListener('mouseup', handleMouseUp);
                progressBarDom.removeEventListener('mouseenter', handleMouseEnter);
                progressBarDom.removeEventListener('mouseleave', handleMouseLeave);
            }
        };
    }

};