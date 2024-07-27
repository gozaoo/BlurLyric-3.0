// base.js
export default {
    debounce(func, wait) {
        let timeoutId = null;

        return function(...args) {
            if (timeoutId) {
                clearTimeout(timeoutId);
            }

            timeoutId = setTimeout(() => {
                func.apply(this, args);
            }, wait);
        };
    }
};
