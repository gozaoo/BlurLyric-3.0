const router =[
    {
        path: '/', component: ()=>import('./routers/home.vue')
    },
    {
        path: '/allmusic/', component: ()=>import('./routers/allMusic.vue')
    },
    {
        path: '/musicFolder/', component: ()=>import('./routers/musicFolder.vue')
    },
    {
        path: '/setting/', component: ()=>import('./routers/setting.vue')
    }
]

export default router