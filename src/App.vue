<template>
  <main
    class="w-full h-screen bg-gray-800 bg-opacity-80 text-white max-h-screen"
  >
    <WindowControl />
    <div
      class="w-full h-[calc(100%-2rem)] flex flex-row items-center justify-center gap-4"
    >
      <MusicDetails v-if="music" v-bind="music" />
      <div class="controls flex flex-col gap-4">
        <UploadFile @contentChanged="onFileContent">选择 TXT 谱</UploadFile>
        <PlayButton
          v-if="music"
          v-bind="music"
          @play="music.play()"
          @stop="music.stop()"
        />
      </div>
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

const music = ref<Music | null>(null);

function onFileContent(content: string) {
  music.value = stringToMusic(content);
}
</script>
