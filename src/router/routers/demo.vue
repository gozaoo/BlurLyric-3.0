<script>
    import contextMenu from '../../components/base/contextMenu.vue';
    import toggle from '../../components/base/toggle.vue'
    import tracksRow from '../../components/tracks/tracksRow.vue'
    import powerTableVue from '../../components/tracks/powerTable_,music.vue';
    import text_contextMenu from '../../components/text_contextMenu.vue';
    import dialogVue from '../../components/base/dialog.vue'
    export default {
        data() {
            return {
                state: true,
                useDialog: false
            }
        },
        components: {
            toggle,
            tracksRow,
            powerTableVue,
            contextMenu,
            text_contextMenu,dialogVue
        },
        inject: ['audioManager','audioState','currentMusicInfo'],
        props: {},
        methods: {

        }
    }
</script>

<template>
    <bodytitle :text="'测试区域'" />
    <bodytitle :text="'音乐测试'"/>

    
    <h2>
        <span style="font-size: 0.8em;">曲名:</span><br>    
        <textspawn :text="currentMusicInfo.name" />
    </h2>
    {{audioState.currentTime}}，{{audioState.duration}}<br>
    进度：{{ ((audioState.currentTime/audioState.duration)*100).toFixed(2) + '%' }}
    
    <iconWithText v-if="audioState.playing==false"   @click="audioManager.play()" type="background">
        <template #text>
            播放
        </template>
        <template #svg>
            <i class="bi bi-play-circle-fill"></i>
        </template>
    </iconWithText>    
    <iconWithText v-if="audioState.playing==true"   @click="audioManager.pause()" type="background">
        <template #text>
            暂停
        </template>
        <template #svg>
            <i class="bi bi-pause-circle-fill"></i>
        </template>
    </iconWithText>

    <bodytitle :text="'组件测试'" />

    <div style="display: flex; gap: 2em;">
        <div>
            <div>
                可用开关
            </div>
            <toggle :type="'normal'" v-model="state"/>
        </div>
        <div>
            <div>
                不可用开关
            </div>
            <toggle :type="'unavailable'" v-model="state" />

        </div>
        
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
    <iconWithText    @click="useDialog= !useDialog" type="background">
        <template #text>
            dialog 对话框启动测试
        </template>
        <template #svg>
            <i class="bi bi-question-circle-fill"></i>
        </template>
    </iconWithText>
    <dialogVue v-if="useDialog">

    </dialogVue>
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