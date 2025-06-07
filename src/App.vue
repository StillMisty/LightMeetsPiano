<template>
  <main
    class="h-screen max-h-screen w-full bg-gray-800/80 text-white"
    @dragover.prevent="isDragging = true"
    @dragleave.prevent="isDragging = false"
    @drop.prevent="onDropOutside"
  >
    <WindowControl />
    <div
      class="flex h-[calc(100%-2rem)] w-full flex-col items-center justify-center gap-4"
    >
      <DragFile v-if="isDragging" @file-dropped="fileDropped"
        >拖放 TXT 谱文件到这里</DragFile
      >
      <template v-else>
        <div class="flex items-center justify-center gap-4">
          <MusicDetails v-if="music" v-bind="music" />
          <div class="controls flex flex-col gap-4">
            <UploadFile @contentChanged="onFileContent">{{ msg }}</UploadFile>
            <PlayButton
              v-if="music"
              v-bind="music"
              @play="music.play()"
              @pause="music.pause()"
            />
          </div>
        </div>
        <ProgressBar
          v-if="music"
          v-bind="music"
          @updateCurrentTime="music.seekTo($event)"
        />
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
import DragFile from "./components/DragFile.vue";
import MusicDetails from "./components/MusicDetails.vue";
import PlayButton from "./components/PlayButton.vue";
import ProgressBar from "./components/ProgressBar.vue";
import UploadFile from "./components/UploadFile.vue";
import WindowControl from "./components/WindowControl.vue";
import { Music, stringToMusic } from "./type/Music";
import { ref } from "vue";

const msg = ref<string>("选择 TXT 谱");
const music = ref<Music | null>(null);

function onFileContent(content: string) {
  try {
    music.value = stringToMusic(content);
    msg.value = "选择 TXT 谱";
  } catch (e) {
    music.value = null;
    msg.value = "非合法 TXT 谱，重新选择";
  }
}

let isDragging = ref(false);

const fileDropped = (txt: string) => {
  onFileContent(txt);
  isDragging.value = false;
};

// 为了防止拖拽时，浏览器默认行为导致的文件打开
const onDropOutside = () => {
  isDragging.value = false;
};
</script>
