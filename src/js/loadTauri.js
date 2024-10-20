
import { Window } from '@tauri-apps/api/window';

const appWindow = new Window('main');


export default {
    lunch() {
        document
            .getElementById('titlebar-minimize') ?
            .addEventListener('click', () => appWindow.minimize());
        document
            .getElementById('titlebar-maximize') ?
            .addEventListener('click', () => appWindow.toggleMaximize());
        document
            .getElementById('titlebar-close') ?
            .addEventListener('click', () => appWindow.close());
    }
}