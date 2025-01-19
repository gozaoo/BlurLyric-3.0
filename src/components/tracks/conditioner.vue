<!--// 父组件的实现方法如下：	<conditioner :condition="arraySortCondition" @conditionChange="arraySortCondition = $event"/>-->
<!--对于arraySortCondition：

			arraySortCondition: {
				// 定义condition对象
				filterFunction: (item) => {
					const content = "";
					return (
						item.name.includes(content) ||
						item.artist.some(artist => artist.name.includes(content)) ||
						item.album.name.includes(content)
					);
				},
				getKey: (item) => item.names,
				sortOrder: 'asc', // 可选参数，'asc'或'desc'
			},
-->
<template>
    <div>
        <iconToClick style="float: left;" type="background" @click="showMore = !showMore">
            <i class="bi bi-funnel"></i>
        </iconToClick>
        &nbsp;
        <input placeholder="搜索包含..." type="text" v-model="content" @input="conditionChange">
        &nbsp;

        <!-- <input type="checkbox" v-model="showMore"><span style="font-size: .8em;"></span> -->
        <span v-if="showMore">
            <span style="font-size: .8em;">根据</span>
            <select v-model="selectedKeyId" @change="conditionChange">
                <option v-for="getKeyMethod in getKeyMethods" :key="getKeyMethod.id" :value="getKeyMethod.id">
                    {{ getKeyMethod.name }}</option>
            </select>
            <select v-model="selectedSortOrder" @change="conditionChange">
                <option value="asc">升序</option>
                <option value="desc">降序</option>
            </select>
            &nbsp;<span style="font-size: .8em;">排序</span>

        </span>

    </div>
    <br>

    <!-- <div>
        <input type="radio" v-model="selectedKeyId" @change="conditionChange" :value="getKeyMethod.id">
        <label>{{ getKeyMethod.name }}</label>
    </div> -->
</template>
<script>
export default {
    props: {

        condition: {
            type: Object,
            required: true
        },
        getKeyMethods: {
            type: Array,
            required: true,
            default: () => [
                {
                    id: 1,
                    name: '歌曲名',
                    method: (item) => item.name
                },
                {
                    id: 2,
                    name: '歌手',
                    method: (item) => item.ar.map(artist => artist.name).join(' ')
                },
                {
                    id: 3,
                    name: '专辑',
                    method: (item) => item.al.name
                },
                {
                    id: 4,
                    name: '专辑序号',
                    method: (item) => String(item.track_number)
                }
            ]
        }
    },
    data() {
        return {
            showMore: false,
            content: '',
            selectedSortOrder: this.condition.sortOrder || 'asc',
            selectedKeyId: this.getKeyMethods[0].id
        };
    },
    computed: {
        selectedKey() {
            return this.getKeyMethods.find(key => key.id === this.selectedKeyId) || this.getKeyMethods[0];
        },
        currentCondition() {
            return {
                filterFunction: (item) => {
                    // const content = this.content;
                    const lowerContent = this.content.toLowerCase();
                    return (
                        (item.name ? item.name.toLowerCase() : '').includes(lowerContent) ||
                        (item.ar || []).some(artist => (artist.name ? artist.name.toLowerCase() : '').includes(lowerContent)) ||
                        (item.al && item.al.name ? item.al.name.toLowerCase() : '').includes(lowerContent)
                    );

                },
                getKey: (item) => {
                    if (typeof this.selectedKey.method === 'function') {
                        return this.selectedKey.method(item);
                    }
                    return '';
                },
                sortOrder: this.selectedSortOrder
            };
        }
    },
    watch: {
        // currentCondition: {
        //     deep: true,
        //     handler(newCondition) {
        //         this.$emit('conditionChange', newCondition);

        //     }
        // }
    },
    emits: ['conditionChange'],
    methods: {
        conditionChange() {

            this.$emit('conditionChange', this.currentCondition);
        }
    }
};
</script>
<style scoped></style>