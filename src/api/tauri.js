// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/core';
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true

// Invoke the command
// invoke('my_custom_command')
// 当使用 Tauri API 的 npm 包时


// 使用命令
export default {
    getMusicList: async ()=>await invoke("get_music_list")
}
