const router =[
    {
        path: '/', component: ()=>import('./routers/home.vue')
    },
    {
        path: '/search/', component: ()=>import('./routers/search.vue')
    },
    {
        path: '/allmusic/', component: ()=>import('./routers/allMusic.vue')
    },
    {
        path: '/musicFolder/', component: ()=>import('./routers/musicFolder.vue')
    },,
    {
        path: '/musicTrack/', component: ()=>import('./routers/musicTrack.vue')
    },
    {
        path: '/setting/', component: ()=>import('./routers/setting.vue')
    },
    {
        path: '/demo/',component:()=>import('./routers/demo.vue')
    },
    {
        path: '/allLocalAlbum/',component:()=>import('./routers/allLocalAlbum.vue')
    },
    {
        path: '/allLocalArtist/',component:()=>import('./routers/allLocalArtist.vue')
    },
    {
        name: 'localArtist',
        path: '/localArtist/',component:()=>import('./routers/localArtist.vue')
    },
    {
        name: 'localAlbum',
        path: '/localAlbum/',component:()=>import('./routers/localAlbum.vue')
    }
]

export default router