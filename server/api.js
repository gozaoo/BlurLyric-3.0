// const express = require('express');
// const router = express.Router()
import express from "express";
const router = express.Router()
import main from "../main.js"

// var cache = {}

// router.get('/searchHan', (req, res) => {
//     const chineseDictionary = main.lib.中文;
//     let tempTime = Date.now()

    
//     let JsonTool = (data)=>{
//         res.json({
//             ...data,
//             takeTime: Date.now() - tempTime
//         })
//     }
//     let result
//     let searchIdiom = () => {
//          result = chineseDictionary.四字成语.data.find((elm) => elm.word.includes(req.query.value));
//         JsonTool({
//             type: 'idiom',
//             result
//         });
//     }

//     switch (req.query.value.length) {
//         case 1:
//             result = chineseDictionary.字.data.find((elm) => elm.word == req.query.value);
//             JsonTool({
//                 type: 'word',
//                 result
//             });
//             break;

//         case 2:
//             result = chineseDictionary.词.data.find((elm) => elm.ci == req.query.value);
//             JsonTool({
//                 type: 'ci',
//                 result
//             });
//             break;

//         case 3:
//         case 4:
//             searchIdiom();
//             break;

//         default:
//             JsonTool({
//                 type: 'undefined',
//             });
//             break;
//     }

// });

export default router