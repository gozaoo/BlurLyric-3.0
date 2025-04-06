<script>
import manager from '../api/manager';
export default {
    data() {
        return {
            step: -1, // -1=预检查 0=欢迎 1=目录设置 2=完成
            loading: true,
            error: null,
            selectedDirs: [],
            existingDirs: [],
            progress: {
                musicList: false,
                folders: false,
                albums: false,
                artists: false
            }
        }
    },
    async mounted() {
        await this.initializeApplication();
    },
    computed: {
        initializationProgress() {
            return Object.values(this.progress).filter(Boolean).length / 4 * 100;
        },
        hasExistingDirs() {
            return this.existingDirs.length > 0;
        }
    },
    methods: {
        async initializeApplication() {
            if (window.__TAURI_INTERNALS__ == true) {
                try {
                    this.step = -1;

                    // 检查现有数据目录
                    const dirs = await manager.tauri.getAllMusicDirs();
                    this.existingDirs = dirs;

                    // 如果已有目录则先初始化应用再加载数据
                    if (this.hasExistingDirs) {
                        // 调用initApplication初始化应用
                        await manager.tauri.initApplication();
                        await this.loadAllData();
                        this.step = 2; // 直接跳转到完成页
                    } else {
                        this.step = 0; // 显示欢迎页
                    }
                } catch (error) {
                    this.handleError(error);
                } finally {
                    this.loading = false;
                }
            } else {
                this.step = 2;
            }
        },

        async loadAllData() {
            try {
                this.loading = true;

                // 并行加载所有数据
                await Promise.all([
                    this.loadData('musicList', manager.tauri.getMusicList()),
                    this.loadData('folders', manager.tauri.getAllMusicDirs()),
                    this.loadData('albums', manager.tauri.getAlbums()),
                    this.loadData('artists', manager.tauri.getArtists())
                ]);

                // 触发缓存更新
            } catch (error) {
                this.handleError(error);
            } finally {
                this.loading = false;
            }
        },

        async loadData(type, promise) {
            try {
                await promise;
                this.progress[type] = true;
            } catch (error) {
                console.error(`Error loading ${type}:`, error);
                throw error;
            }
        },

        async addDirectories() {
            if (this.selectedDirs.length === 0) return;

            try {
                this.loading = true;
                await manager.tauri.addMusicDirs(this.selectedDirs);
                await this.loadAllData();
                this.step++;
            } catch (error) {
                this.handleError(error);
            } finally {
                this.loading = false;
            }
        },

        async skipSetup() {
            this.step = 2;
            // 确保在跳过设置时也调用initApplication
            await manager.tauri.initApplication();
            await this.loadAllData();
        },

        handleError(error) {
            console.error('OOBE Error:', error);
            this.error = error.message || '发生未知错误';
            setTimeout(() => this.error = null, 5000);
        }
    },
    inject: ['source']
}
</script>

<template>
    <div class="oobeContainer" v-show="step !== 2">
        <!-- 预加载界面 -->
        <div v-if="step === -1" class="initializing">
            <h1>正在初始化...</h1>
            <div class="progress-bar">
                <div class="progress" :style="{ width: initializationProgress + '%' }"></div>
            </div>
            <p>已加载数据：</p>
            <ul>
                <li :class="{ loaded: progress.musicList }">音乐列表</li>
                <li :class="{ loaded: progress.folders }">目录结构</li>
                <li :class="{ loaded: progress.albums }">专辑数据</li>
                <li :class="{ loaded: progress.artists }">艺术家数据</li>
            </ul>
        </div>

        <!-- 欢迎页面 -->
        <div v-if="step === 0" class="welcome">
            <h1>欢迎使用 BlurLyric</h1>
            <p>一款现代化的开源音乐播放器</p>
            <div class="features">
                <p>✓ 本地音乐管理</p>
                <p>✓ 智能歌词显示</p>
                <p>✓ 跨平台支持</p>
            </div>
            <button @click="step++">开始设置</button>
            <button v-if="hasExistingDirs" @click="skipSetup" class="text-button">使用现有配置</button>
        </div>

        <!-- 目录设置 -->
        <div v-if="step === 1" class="directory-setup">
            <h1>音乐目录设置</h1>
            <p>请选择包含音乐文件的文件夹：</p>

            <div class="directory-list">
                <div v-for="dir in source.local.folders.data" :key="dir" class="directory-item">
                    {{ dir }}
                </div>
            </div>

            <div class="actions">
                <button @click="selectedDirs = []">重新选择</button>
                <button @click="addDirectories">添加目录</button>
            </div>

            <div v-if="error" class="error-message">
                {{ error }}
            </div>

            <button @click="step--" class="text-button">返回</button>
        </div>

        <!-- 加载状态 -->

    </div>
</template>

<style scoped>
.oobeContainer {
    position: fixed;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    width: 100vw;
    background: var(--background-bar);
    left: 0;
    visibility: visible;
    opacity: 1;
    /* transition: 1s; */
    top: 0;
    right: 0;
    bottom: 0;
    z-index: 999;
}

.progress-bar {
    width: 200px;
    height: 8px;
    background: #eee;
    border-radius: 4px;
    margin: 20px 0;
}

.progress {
    height: 100%;
    background: var(--primary-color);
    border-radius: 4px;
    transition: width 0.3s ease;
}

.features p {
    margin: 8px 0;
    color: #666;
}

.directory-list {
    max-height: 200px;
    overflow-y: auto;
    margin: 20px 0;
}

.directory-item {
    padding: 8px;
    margin: 4px 0;
    background: #f5f5f5;
    border-radius: 4px;
}

.actions {
    margin-top: 20px;
    display: flex;
    gap: 10px;
    justify-content: center;
}

button {
    padding: 10px 20px;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}

.text-button {
    background: none;
    color: var(--primary-color);
    text-decoration: underline;
}




.error-message {
    color: #ff4444;
    margin-top: 10px;
}
</style>