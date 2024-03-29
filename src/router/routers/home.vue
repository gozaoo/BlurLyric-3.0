<script setup>
import flexColumnRow from '../../components/flexColumnRow.vue';
import musicCard from '../../components/musicCard.vue';


import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()
</script>

<template>
    <h1>音乐库</h1>
    <flexColumnRow>
        <musicCard @click="router.push('/allMusic/')" title="本地音乐" type="gradient"></musicCard>
    </flexColumnRow>
</template>

<style scoped>
</style>