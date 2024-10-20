// 导入Vue和Vue Router库
import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';

import { Window } from '@tauri-apps/api/window';

const appWindow = new Window('main');

document
  .getElementById('titlebar-minimize')
  ?.addEventListener('click', () => appWindow.minimize());
document
  .getElementById('titlebar-maximize')
  ?.addEventListener('click', () => appWindow.toggleMaximize());
document
  .getElementById('titlebar-close')
  ?.addEventListener('click', () => appWindow.close());

// 导入应用程序样式和Vue组件
import './style.css';
import App from './App.vue';
import 'bootstrap-icons/font/bootstrap-icons.css';
import iconToClickVue from './components/iconToClick.vue';
import iconWithTextVue from './components/iconWithText.vue';
import toggleVue from './components/base/toggle.vue';
// import toggleVue from './components/toggle.vue';
import textspawnVue from './components/base/text-spawn.vue';
import bodytitleVue from './components/base/bodytitle.vue'

// 导入路由配置
import routes from './router/index.js';

// 创建Vue应用实例
let vueApp = createApp(App);
 
// 注册全局组件（链式调用）
vueApp.component('iconToClick', iconToClickVue)
       .component('iconWithText', iconWithTextVue)
       .component('toggle', toggleVue)
       .component('bodytitle', bodytitleVue)
       .component('textspawn',textspawnVue)


// 创建Vue Router实例并配置路由
const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

// 使用路由插件
vueApp.use(router);

// 挂载Vue应用实例到DOM元素
vueApp.mount('#app');

export default vueApp
