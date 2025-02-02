<template>
  <button
    class="rounded-lg border-2 border-slate-400 p-3 transition hover:bg-slate-50 hover:text-gray-800"
    @click="props.isplay ? emit('pause') : emit('play')"
  >
    {{ props.isplay ? "暂停" : "播放" }}
  </button>
</template>

<script lang="ts" setup>
const props = defineProps<{
  isplay: boolean;
}>();
const emit = defineEmits(["play", "pause"]);
</script>
