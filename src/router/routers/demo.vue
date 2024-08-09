<script>
    import contextMenu from '../../components/base/contextMenu.vue';
    import toggle from '../../components/base/toggle.vue'
    import tracksRow from '../../components/tracks/tracksRow.vue'
    import powerTableVue from '../../components/tracks/powerTable.vue'
    import text_contextMenu from '../../components/text_contextMenu.vue';
    export default {
        data() {
            return {
                state: true
            }
        },
        components: {
            toggle,
            tracksRow,
            powerTableVue,
            contextMenu,
            text_contextMenu
        },
        props: {},
        methods: {

        }
    }
</script>

<template>
    <bodytitle :text="'测试区域'" />
    <bodytitle :text="'组件测试'" />

    <div>
        可用开关
        <toggle :type="'normal'" :state="state" @changeState="this.state==!this.state" />
        不可用开关
        <toggle :type="'unavailable'" :state="true" @changeState="this.state==!this.state" />
    </div>
    <br>
    <contextMenu :menuItems="[
        { name: '这是一个未封装完全的contextMenu测试组件', handleClick: () => { console.log('点击了菜单项1'); } },
        { name: '菜单项2', handleClick: () => { console.log('点击了菜单项2'); } }
      ]
    ">
        <iconWithText type="background">
            <template #text>
                右键菜单测试区域
            </template>
            <template #svg>
                <i class="bi bi-menu-button-fill"></i>
            </template>
        </iconWithText>
    </contextMenu>
    <br>

    <text_contextMenu>
        <iconWithText type="background">
            <template #text>
                文本复制模板区域测试
            </template>
            <template #svg>
                <i class="bi bi-blockquote-left"></i>
            </template>
        </iconWithText>

    </text_contextMenu>
    <br>

    <powerTableVue :tableData="{cellArray: [{
							name: '音乐列表复制测试',
							artist: '音乐人名',
							album: '专辑名123',
							imgSrc: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',
							trackOrdinalNumber: '1',
							duration: '02:02',
						}]}">
    </powerTableVue>

    <bodytitle :text="'标题变化控制组件'" />
    <bodytitle :text="'测试1'" />
    <bodytitle :text="'测试2'" />
    <br>
    <br>
    <br>
    <bodytitle :text="'测试3'" />

    <text_contextMenu>
        
    <h2>列表组件</h2>
    </text_contextMenu>
    <div>
        <powerTableVue :tableData="{cellArray: [{
							name: '时间线',
							artist: 'HOYO-MIX',
							album: '崩坏星穹铁道-失控 Out of Control',
							imgSrc: 'http://p1.music.126.net/RWIGyShmnjmUxizXco6fVg==/109951168505830245.jpg',
							trackOrdinalNumber: '1',
							duration: '02:02',
						}]}">
        </powerTableVue>
    </div>
</template>

<style scoped>
    .buttomTrack {
        display: flex;

    }

    .buttomTrack>* {
        width: fit-content;
    }
</style>