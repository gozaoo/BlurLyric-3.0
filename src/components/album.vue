<template>
    <div ref="albumDom" class="album">
        <div class="coverImageRow">
            <div >
                <LazyLoadCoverImage class="blur" :id="album.id"></LazyLoadCoverImage>
                <LazyLoadCoverImage :id="album.id"></LazyLoadCoverImage>
            </div>
        </div>

        <h3 class="name" v-if="noTitle == false">
            {{ album.name }}
        </h3>
    </div>
</template>

<script>
import LazyLoadCoverImage from './base/lazyLoadCoverImage.vue'
export default {
    name: 'album',
    props: {
        album: Object,
        noTitle: {
            default: false,
            type: Boolean
        }
    },
    components: {
        LazyLoadCoverImage
    },
}
</script>

<style scoped>
*{
    user-select: none;
    /* pointer-events: ; */
    cursor: pointer;
}
.coverImageRow {
    aspect-ratio: 1 / 1;
    position: relative;
}

.coverImageRow .image-container {
    aspect-ratio: 1 / 1;
}

.coverImageRow .blur {
    filter: blur(2em) contrast(4) ;
    position: absolute;
    width: 100%;
    height: 100%;
    transform-origin: 50% 100%;
    transform: scale(0.85);
}
h3{
    /* 不能超过两行*/
    -webkit-line-clamp: 2;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    overflow: hidden;

    font-size: min(0.01vw, 1.5em);
}

</style>