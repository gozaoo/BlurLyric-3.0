<template>
    <div ref="albumDom" class="album">
        <div class="coverImageRow">
            <div >
                <LazyLoadCoverImage class="blur" :id="album.id"></LazyLoadCoverImage>
                <LazyLoadCoverImage :id="album.id"></LazyLoadCoverImage>
            </div>
        </div>
        <div>

        </div>
        <h3>
            {{ album.name }}
        </h3>
    </div>
</template>

<script>
import LazyLoadCoverImage from './base/lazyLoadCoverImage.vue'
export default {
    name: 'album',
    props: {
        album: Object
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
    filter: blur(2em) contrast(4) brightness(0.8);
    position: absolute;
    width: 100%;
    height: 100%;
    transform-origin: 50% 100%;
    transform: scale(0.85);
}

</style>