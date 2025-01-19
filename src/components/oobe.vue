<script>
import manager from '../api/manager.js';

export default {
    data() {
        return {
            currentText: '',
            allMusicDirs: [],
            allMusicList: [],
            step: 0,
            /**
             * 0: 拉取缓存
             * 1. 引导增加目录
             * 2. 引导增加目录完成
             * 3: 扫描音乐
             * 4: 扫描音乐完成
             * 5. 检查完毕
             */
        }
    },
    async mounted() {
        this.currentText = '拉取缓存';
        this.allMusicList = await manager.tauri.getMusicList();
        this.getAllMusicDirs = await manager.tauri.getAllMusicDirs();

        // 如果均为空，则尝试初始化
        if(this.allMusicList.length === 0 && this.allMusicDirs.length === 0) {
            this.currentText = '尝试初始化并扫描音乐';
            await manager.tauri.initApplication();
        }
        

        if (this.allMusicDirs.length === 0) {
            currentText = '请添加你的音乐目录';
            this.step = 1;
        }
    }
}
</script>
<template>

</template>