export default {
    /**
     * 
     * @param {*} array 
     * @param {object} condition 
     * @returns array
     * 
     * using example:
     * 
    * // 定义condition对象
        const condition = {
        filterFunction: (item) => {
            const content = "小星星";
            return (
            item.name.includes(content) ||
            item.artist.some(artist => artist.name.includes(content)) ||
            item.album.name.includes(content)
            );
        },
        getKey: (item) => item.names,
        sortOrder: 'asc', // 可选参数，'asc'或'desc'
        };

        // 调用函数
        const newArray = filterAndSort(array, condition);

        // 输出结果
        console.log(newArray);
     */
    filterAndSort(array, condition) {
        // 确保array是一个数组且condition对象存在
        if (!Array.isArray(array) || typeof condition !== 'object') {
            throw new TypeError('Invalid arguments: array must be an array and condition must be an object.');
        }

        // 确保filterFunction和getKey是函数
        const filterFunction = condition.filterFunction;
        if (typeof filterFunction !== 'function') {
            throw new TypeError('Condition must include a filterFunction property that is a function.');
        }

        const getKey = condition.getKey;
        if (typeof getKey !== 'function') {
            throw new TypeError('Condition must include a getKey property that is a function.');
        }

        // 确定排序顺序，默认为升序
        const sortOrder = condition.sortOrder || 'asc';

        // 过滤数组
        const filteredArray = array.filter(filterFunction);

        // 辅助函数：判断键值是否以数字开头
        const isNumberStarting = (key) => typeof key === 'string' && /^\d/.test(key);

        // 辅助函数：提取数字值
        const getNumericValue = (key) => parseInt(key, 10);

        // 定义排序函数
        const sortFunction = (a, b) => {
            const keyA = getKey(a) || '';
            const keyB = getKey(b) || '';

            const isNumA = isNumberStarting(keyA);
            const isNumB = isNumberStarting(keyB);

            if (sortOrder === 'asc') {
                if (!isNumA && isNumB) {
                    return -1; // 字母在前，数字在后
                } else if (isNumA && !isNumB) {
                    return 1; // 数字在字母之后
                } else if (!isNumA && !isNumB) {
                    // 两者都是字母，按字母顺序排序，包含拼音
                    return keyA.localeCompare(keyB, 'zh-CN');
                } else if (isNumA && isNumB) {
                    // 两者都是数字，按数值排序
                    return getNumericValue(keyA) - getNumericValue(keyB);
                }
            } else if (sortOrder === 'desc') {
                if (isNumA && !isNumB) {
                    return -1; // 数字在前，字母在后
                } else if (!isNumA && isNumB) {
                    return 1; // 字母在数字之后
                } else if (!isNumA && !isNumB) {
                    // 两者都是字母，按 reverse 字母顺序排序，包含拼音
                    return keyB.localeCompare(keyA, 'zh-CN');
                } else if (isNumA && isNumB) {
                    // 两者都是数字，按数值降序排序
                    return getNumericValue(keyB) - getNumericValue(keyA);
                }
            } else {
                throw new Error('Invalid sortOrder. Must be "asc" or "desc".');
            }
        };

        // 排序过滤后的数组
        const sortedArray = [...filteredArray].sort(sortFunction);

        return sortedArray;
    },
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
    sortByTrackNumber(myarray) {
        myarray.sort(function (a, b) {
            // 假设track_number是一个数字，如果不是，可能需要根据实际情况调整比较逻辑
            return a.track_number - b.track_number;
        });
    },
    formatTime_MMSS(seconds) {
        if (typeof seconds !== 'number' || seconds < 0) {
            return 'Invalid input';
        }

        const minutes = Math.floor(seconds / 60);
        const remainingSeconds = seconds % 60;

        const paddedMinutes = minutes.toString().padStart(2, '0');
        const paddedSeconds = remainingSeconds.toString().padStart(2, '0');

        return `${paddedMinutes}:${paddedSeconds}`;
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
                info.beforeDragProgress = getCurrentProgress();

                document.addEventListener('touchmove', handleTouchMove);
                document.addEventListener('touchend', handleTouchEnd);
                info.draging = true;
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
                info.dragProgress = (x - rect.left - info.offsetX) / info.domWidth;
                info.currentProgress = info.beforeDragProgress + info.dragProgress;
                onInfoChange();
            }
        };

        const handleTouchEnd = () => {
            if (info.draging) {
                info.draging = false;
                info.currentProgress = info.beforeDragProgress + info.dragProgress;
                onInfoChange();
            }
        };

        const handleMouseDown = (e) => {
            info.beforeDragProgress = getCurrentProgress();

            info.draging = true;

            document.addEventListener('mousemove', handleMouseMove);
            document.addEventListener('mouseup', handleMouseUp);
            const rect = progressBarDom.getBoundingClientRect();
            info.offsetX = e.clientX - rect.left;
            info.domWidth = rect.width;
            onInfoChange();
        };

        const handleMouseMove = (e) => {
            if (info.draging) {
                const x = e.clientX;
                const rect = progressBarDom.getBoundingClientRect();
                info.dragProgress = (x - rect.left - info.offsetX) / info.domWidth;
                info.currentProgress = info.beforeDragProgress + info.dragProgress;
                onInfoChange();
            }
        };

        const handleMouseUp = (e) => {
            if (info.draging) {
                info.draging = false;
                info.currentProgress = info.beforeDragProgress + info.dragProgress;
                onInfoChange();
            }
        };

        const handleMouseEnter = (e) => {
            info.hovering = true;
            info.BeforeHoveringProgress = info.currentProgress;
            progressBarDom.addEventListener('mouseleave', handleMouseLeave);
            onInfoChange();
        };

        const handleMouseLeave = (e) => {
            info.hovering = false;
            info.currentProgress = info.BeforeHoveringProgress;
            onInfoChange();
        };

        progressBarDom.addEventListener('touchstart', handleTouchStart);
        progressBarDom.addEventListener('mousedown', handleMouseDown);
        progressBarDom.addEventListener('mouseenter', handleMouseEnter);


        return {
            cancelReg: () => {
                progressBarDom.removeEventListener('touchstart', handleTouchStart);
                document.removeEventListener('touchmove', handleTouchMove);
                document.removeEventListener('touchend', handleTouchEnd);
                progressBarDom.removeEventListener('mousedown', handleMouseDown);
                document.removeEventListener('mousemove', handleMouseMove);
                document.removeEventListener('mouseup', handleMouseUp);
                progressBarDom.removeEventListener('mouseenter', handleMouseEnter);
                progressBarDom.removeEventListener('mouseleave', handleMouseLeave);
            }
        };
    }

};