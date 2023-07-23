// const express = require('express')
import express from 'express'

const app = express()
const port = 18775 // 自定义端口号（不要与已存在端口冲突）


app.use(express.static('dist')) // dist 是项目的打包资源
app.use('/api', api)

import { createServer } from 'vite'

const server = await createServer({
  // 任何合法的用户配置选项，加上 `mode` 和 `configFile`
  configFile: './vite.config.js',
  root: './',
})
server.listen()
async function main(){
  let toDoList = [
    showWelcome
  ]

  for (let index = 0; index < toDoList.length; index++) {
    const element = toDoList[index];
    await element()
    // console.log('加载' + index +'成功')
  }

  app.listen(port, () => {
    console.log(`服务器已经运行在了 http://localhost:${port}`)
  })
}
main()




async function showWelcome(){
  let welcome = [
    '==================================',
    'BlurLyric 3.0 后端服务启动中...',
    '',
  ]

  for (let i = 0;i < welcome.length; i++) {
    console.log(welcome[i])
  }

}


export default datas