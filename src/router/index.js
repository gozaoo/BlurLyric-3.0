const router =[
    {
        path: '/', component: ()=>import('./routers/home.vue')
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
    }
]

export default router