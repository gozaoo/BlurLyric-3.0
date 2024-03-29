// lyricParser.js
export default {
    parseYRClyric(contentText) {
        const lyrics = {
            type: "yrc",
            headers: {},
            lines: []
        };
        // 按行分割歌词文本
        const lines = contentText.split('\n');
        // 解析头部信息
        let headerEndIndex = lines.findIndex(line => line.startsWith('['));
        lines.slice(0, headerEndIndex).forEach(headerLine => {
            try {
                const header = JSON.parse(headerLine);
                const key = header.c[0].tx.replace(/:$/, '');
                const value = header.c.slice(1).map(item => item.tx).join('/');
                lyrics.headers[key] = value;
            } catch (e) {
                console.error('解析头部信息出错:', e);
            }
        });
        // 解析歌词
        lines.slice(headerEndIndex).forEach(line => {
            if (line.startsWith('[')) {
                const timestamps = line.match(/\[(\d+),(\d+)\]/);
                const startTime = Number(timestamps[1]) / 1000;
                const duration = Number(timestamps[2]) / 1000;
                const words = [];

                line.replace(/\((\d+),(\d+),\d+\)([^\(]*)/g, (_, wordStart, wordDuration, word) => {
                    let startTime = Number(wordStart) / 1000,
                        duration = Number(wordDuration) / 1000;
                    let endTime = Number((startTime + duration).toFixed(3))

                    if(word=='')return'';
                    words.push({
                        startTime,
                        duration,
                        endTime,
                        word
                    });
                    return '';
                });
                let text = ''
                words.forEach(element => {
                    text+=element.word
                });
                lyrics.lines.push({
                    startTime:words[0].startTime,
                    duration,
                    endTime: words[words.length-1].endTime,
                    words,
                    text
                });
            }
        });
        return lyrics;
    },
    parseLRClyric(content) {
        // 初始化歌词数组和一个用于存储元数据的对象
        const lyrics = [];
        const headers = {};

        // 将歌词文本按行分割
        const lines = content.split('\n');
        // 初始化上一个时间戳为0
        let previousTime = 0;

        // 遍历每一行歌词
        lines.forEach(line => {
            // 如果行以'['开头，则可能是时间戳或元数据标签
            if (line.startsWith('[')) {
                // 使用正则表达式匹配时间戳
                const timestampMatch = line.match(/\[(\d*):(\d*)\.(\d*)\]/g);
                if (timestampMatch) {
                    // 从行中移除时间戳并修剪空白字符
                    const text = line.replace(timestampMatch.join(''), '').trim();
                    // 遍历每个时间戳
                    timestampMatch.forEach(timestamp => {
                        // 解构时间戳的分钟、秒和毫秒
                        const [_, mm, ss, sss] = timestamp.match(/(\d*):(\d*)\.(\d*)/);
                        // 计算时间戳的总秒数
                        const time = (Number(mm) * 60) + Number(ss) + (Number(sss) / 1000);
                        // 如果行包含歌词文本，则添加到歌词数组
                        if (text) {
                            lyrics.push({
                                startTime: time,
                                endTime: 0,
                                text,
                                duration: 0
                            });
                        }
                        // 更新上一个时间戳为当前时间戳（取最大值以处理乱序时间戳）
                        previousTime = Math.max(previousTime, time);
                    });
                } else {
                    // 如果不是时间戳，则可能是元数据标签
                    const [key, value] = line.replace('[', '').replace(']', '').split(':');
                    // 避免重复标签，只有当元数据对象中不存在该键时才添加
                    if (!headers.hasOwnProperty(key)) {
                        headers[key] = value.trim();
                    }
                }
            } else if (line.trim() === '') {
                // 如果是空白行，表示较长的音乐间隔
                if (lyrics.length > 0) {
                    // 获取最后一行歌词
                    const lastLyric = lyrics[lyrics.length - 1];
                    // 如果最后一行歌词的结束时间未设置，则设置为上一个时间戳
                    if (lastLyric.endTime === 0) {
                        lastLyric.endTime = previousTime;
                        lastLyric.duration = lastLyric.endTime - lastLyric.startTime;
                    }
                }
            }
        });

        // 如果歌词数组不为空，处理最后一行歌词的结束时间和时长
        if (lyrics.length > 0) {
            const lastLyric = lyrics[lyrics.length - 1];
            if (lastLyric.endTime === 0) {
                lastLyric.endTime = previousTime;
            }
            lastLyric.duration = lastLyric.endTime - lastLyric.startTime;
        }

        // 计算每行歌词的结束时间和时长
        for (let i = 1; i < lyrics.length; i++) {
            const currentLyric = lyrics[i];
            const previousLyric = lyrics[i - 1];
            previousLyric.endTime = currentLyric.startTime;
            previousLyric.duration = previousLyric.endTime - previousLyric.startTime;
        }

        // 返回元数据和歌词数组
        return {
            type: 'lrc',
            headers,
            lines:lyrics
        };

    }
}