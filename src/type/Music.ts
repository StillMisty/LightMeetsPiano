import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

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
  pendingTimeouts: number[] = []; // 待执行的任务
  currentTime: number = 0; // 当前播放时间 单位秒
  duration: number = 0; // 曲谱时长
  handlePlay: number | null = null; // 播放句柄
  isplay: boolean = false; // 是否正在播放
  private playLock: boolean = false; // 播放锁，防止重复播放

  constructor(
    name: string,
    author: string,
    transcribedBy: string,
    bpm: number,
    pitchLevel: number,
    songNotes: Note[],
  ) {
    this.name = name;
    this.author = author;
    this.transcribedBy = transcribedBy;
    this.bpm = bpm;
    this.pitchLevel = pitchLevel;
    // 按照时间排序
    songNotes.sort((a, b) => a.time - b.time);
    this.songNotes = songNotes;
    this.duration = songNotes[songNotes.length - 1].time / 1000;
  }

  async play() {
    console.log("play");
    // 播放曲谱

    this.isplay = true;

    if (this.playLock) {
      return;
    }
    this.playLock = true;
    // 等待窗口失去焦点
    await this.waitLostFocus();

    // 如果在等待的过程中，isplay 变为 false，说明用户点击了停止按钮，直接返回
    if (!this.isplay) {
      return;
    }

    // 设置播放句柄
    this.clearPlayHandle();
    this.setPlayHandle();

    // 播放每个音符
    this.clearPendingTimeouts();
    for (const note of this.songNotes) {
      if (note.time < this.currentTime * 1000) {
        continue;
      }
      this.pendingTimeouts.push(
        setTimeout(
          () => {
            invoke("press_key", { key: note.key });
          },
          note.time - this.currentTime * 1000,
        ),
      );
    }

    this.playLock = false;
  }

  pause() {
    // 暂停播放
    this.isplay = false;
    // 清除定时器
    this.clearPendingTimeouts();
    // 清除播放句柄
    this.clearPlayHandle();
  }

  async seekTo(time: number) {
    // 跳转到指定时间播放
    this.currentTime = time;
    this.pause();
  }

  // 保存曲谱
  store() {}

  private clearPlayHandle() {
    // 清除播放句柄
    if (this.handlePlay) {
      clearInterval(this.handlePlay);
    }
  }

  private setPlayHandle() {
    this.handlePlay = setInterval(() => {
      this.currentTime += 1;
      if (this.currentTime >= this.duration) {
        this.pause();
        this.currentTime = 0;
      }
    }, 1000);
  }

  private clearPendingTimeouts() {
    // 清除所有待执行的任务
    for (const timeout of this.pendingTimeouts) {
      clearTimeout(timeout);
    }
  }
  private async waitLostFocus() {
    return new Promise<void>((resolve) => {
      const handleFocus = setInterval(async () => {
        if (await getCurrentWindow().isFocused()) {
          clearInterval(handleFocus);
          resolve();
        }
      }, 100);
    });
  }
}

function stringToMusic(jsonString: string): Music | null {
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
    parsedData.songNotes,
  );
  return music;
}

export { stringToMusic, Music };
