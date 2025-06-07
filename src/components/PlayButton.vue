<template>
  <button
    class="cursor-pointer rounded-lg border-2 border-slate-400 p-3 transition hover:bg-slate-50 hover:text-gray-800 focus:outline-none"
    @click="props.isPlay ? emit('pause') : emit('play')"
  >
    {{ props.isPlay ? "暂停" : "播放" }}
  </button>
</template>

<script lang="ts" setup>
const props = defineProps<{
  isPlay: boolean;
}>();
const a = 1;
if (a === 1) {
  console.log("a is 1");
}
const emit = defineEmits(["play", "pause"]);
</script>
