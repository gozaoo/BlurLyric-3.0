<script>

    import toggle from '../../components/base/toggle.vue'
    import tracksRow from '../../components/tracks/tracksRow.vue'
	import powerTableVue from '../../components/tracks/powerTable.vue'

    export default {
        data() {
            return {
                state: true
            }
        },
        components: {
            toggle,
            tracksRow,
			powerTableVue
        },
        props: {
        },
        methods: {

        }
    }
</script>

<template>
    <h1>组件测试</h1>
    <h2>基础组件</h2>
    <div>
        可用开关
        <toggle :type="'normal'" :state="state" @changeState="this.state==!this.state"/>
        不可用开关
		<toggle :type="'unavailable'" :state="true" @changeState="this.state==!this.state"/>
    </div>
    <h2>列表组件</h2>
    <div>
		<powerTableVue>
			
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