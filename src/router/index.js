const router =[
    {
        path: '/', component: ()=>import('./routers/home.vue')
    },
    {
        path: '/allmusic/', component: ()=>import('./routers/allMusic.vue')
    }
]

export default router