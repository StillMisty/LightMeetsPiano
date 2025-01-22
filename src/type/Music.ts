import { invoke } from "@tauri-apps/api/core";

interface Note {
  time: number; // 时间
  key: string; // 音符
}

class Music {
  name: string; // 曲谱名称
  author: string; // 作者
  transcribedBy: string; // 谱曲者
  bpm: number; // 每分钟节拍数
  pitchLevel: number; // 音高
  songNotes: Note[]; // 音符
  isplay: boolean = false; // 是否正在播放
  pendingTimeouts: number[] = []; // 待执行的任务

  constructor(
    name: string,
    author: string,
    transcribedBy: string,
    bpm: number,
    pitchLevel: number,
    songNotes: Note[]
  ) {
    this.name = name;
    this.author = author;
    this.transcribedBy = transcribedBy;
    this.bpm = bpm;
    this.pitchLevel = pitchLevel;
    // 按照时间排序
    songNotes.sort((a, b) => a.time - b.time);
    this.songNotes = songNotes;
  }

  async play() {
    // 播放曲谱
    // 1. 等待俩秒
    // 2. 播放每个音符
    this.isplay = true;

    // 等待三秒
    await new Promise((resolve) => setTimeout(resolve, 2000));
    // 如果在等待的过程中，isplay 变为 false，说明用户点击了停止按钮，直接返回
    if (!this.isplay) {
      return;
    }

    // 播放每个音符
    this.pendingTimeouts = [];
    for (let i = 0; i < this.songNotes.length; i++) {
      const note = this.songNotes[i];
      this.pendingTimeouts.push(
        setTimeout(() => {
          invoke("press_key", { key: note.key });

          // 如果是最后一个音符，设置 isplay 为 false
          if (i === this.songNotes.length - 1) {
            this.isplay = false;
          }
        }, note.time)
      );
    }
  }

  stop() {
    // 暂停播放
    this.isplay = false;
    // 清除所有待执行的任务
    for (const timeout of this.pendingTimeouts) {
      clearTimeout(timeout);
    }
  }

  // 保存曲谱
  store() {}
}

function stringToMusic(jsonString: string): Music | null {
  try {
    // 1. 解析 JSON 字符串
    const parsedData = JSON.parse(jsonString)[0];

    // 2. 校验解析后的数据类型，确保数据的完整性和正确性
    if (
      typeof parsedData !== "object" ||
      parsedData === null ||
      typeof parsedData.name !== "string" ||
      typeof parsedData.author !== "string" ||
      typeof parsedData.transcribedBy !== "string" ||
      typeof parsedData.bpm !== "number" ||
      typeof parsedData.pitchLevel !== "number" ||
      !Array.isArray(parsedData.songNotes)
    ) {
      console.error("Invalid JSON structure for Music object");
      console.log(parsedData);
      return null;
    }

    // 3. 校验 songNotes 数组中元素的数据类型
    for (const note of parsedData.songNotes) {
      if (
        typeof note !== "object" ||
        note === null ||
        typeof note.time !== "number" ||
        typeof note.key !== "string"
      ) {
        console.error("Invalid JSON structure for Note in Music.songNotes");
        return null;
      }
    }

    // 3. 创建 Music 对象
    const music = new Music(
      parsedData.name,
      parsedData.author,
      parsedData.transcribedBy,
      parsedData.bpm,
      parsedData.pitchLevel,
      parsedData.songNotes
    );
    return music;
  } catch (error) {
    console.error("Error converting string to Music:", error);
    return null;
  }
}

export { stringToMusic, Music };
