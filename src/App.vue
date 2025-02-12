<template>
  <main class="h-screen max-h-screen w-full bg-gray-800/80 text-white">
    <WindowControl />
    <div
      class="flex h-[calc(100%-2rem)] w-full flex-col items-center justify-center gap-4"
    >
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
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Music, stringToMusic } from "./type/Music";
import UploadFile from "./components/UploadFile.vue";
import MusicDetails from "./components/MusicDetails.vue";
import PlayButton from "./components/PlayButton.vue";
import WindowControl from "./components/WindowControl.vue";
import ProgressBar from "./components/ProgressBar.vue";

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
</script>
