<template>
  <div
    class="h-screen w-screen bg-background/75 rounded-md flex flex-col"
    @dragover.prevent="isDragging = true"
    @dragleave.prevent="isDragging = false"
    @drop.prevent="onDropOutside"
  >
    <WindowControl class="h-8 shrink-0" />
    <main class="flex content-center justify-center flex-col size-full p-2">
      <DragFile v-if="isDragging" @file-dropped="fileDropped"
        >拖放 TXT 谱文件到这里</DragFile
      >
      <template v-else>
        <template v-if="music">
          <div
            class="grid grid-cols-[3fr_2fr] grid-rows-2 gap-2 mb-1 size-full"
          >
            <MusicDetails v-bind="music" class="row-span-2" />
            <UploadFile @contentChanged="onFileContent" class="size-full"
              >选择TXT谱</UploadFile
            >
            <PlayButton
              :is-playing="music.isPlay"
              @play="music.play()"
              @pause="music.pause()"
              class="size-full"
            />
          </div>
          <ProgressBar
            v-bind="music"
            @updateCurrentTime="music.seekTo($event)"
          />
        </template>
        <div v-else class="flex justify-center items-center">
          <UploadFile @contentChanged="onFileContent" class="size-full max-w-32"
            >选择TXT谱</UploadFile
          >
        </div>
      </template>
    </main>
    <Toaster />
  </div>
</template>

<script setup lang="ts">
import DragFile from "./components/DragFile.vue";
import MusicDetails from "./components/MusicDetails.vue";
import PlayButton from "./components/PlayButton.vue";
import ProgressBar from "./components/ProgressBar.vue";
import UploadFile from "./components/UploadFile.vue";
import WindowControl from "./components/WindowControl.vue";
import { Music, stringToMusic } from "./type/Music";
import { Toaster } from "@/components/ui/sonner";
import { ref } from "vue";
import { toast } from "vue-sonner";
import "vue-sonner/style.css";

const music = ref<Music | null>(null);

function onFileContent(content: string) {
  music.value = stringToMusic(content);
  if (!music.value) {
    toast.error("无效TXT谱");
  }
  isDragging.value = false;
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
