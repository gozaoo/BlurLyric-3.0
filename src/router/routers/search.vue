<template>
    <bodytitle :text="'搜索'" />

    <div>
            <input style="width: 210px" type="text"   v-model="addLocalDirInputValue">
            <!-- <input v-model="query" @input="search"/> -->
        <div v-if="results.length">
            <ul>
                <li v-for="result in results" :key="result.id">{{ result.name }}</li>
            </ul>
        </div>
        <div v-else>
            No results found.
        </div>
    </div>
</template>

<script>
import { inject } from 'vue';

export default {
    name: 'Search',
    data() {
        return {
            query: '',
            results: []
        };
    },
    setup() {
        const source = inject('source');
        return { source };
    },
    methods: {
        async search() {
            if (this.query.trim() === '') {
                this.results = [];
                return;
            }

            // Example search logic, replace with actual search logic
            const searchResults = await this.source.search(this.query);
            this.results = searchResults;
        }
    }
};
</script>

<style scoped>


ul {
    list-style-type: none;
    padding: 0;
}

li {
    padding: 5px 0;
}
</style>