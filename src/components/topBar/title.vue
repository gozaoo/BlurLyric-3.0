<template>
    <div class="title">
        <h1>{{ title }}</h1>
        <p>{{ description }}</p>
    </div>
</template>
<script>
    export default{
        data(){
            return {
                
            }
        },
        inject:['title']
    }
</script>