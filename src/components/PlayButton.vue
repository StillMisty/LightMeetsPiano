<template>
  <Button
    @click="togglePlay"
    :variant="isPlaying ? 'destructive' : 'outline'"
    class="cursor-pointer border-2 border-slate-400! hover:backdrop-brightness-200"
  >
    <component :is="isPlaying ? Pause : Play" class="size-4" />
    {{ isPlaying ? "暂停" : "播放" }}
  </Button>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Play, Pause } from "lucide-vue-next";
import { ref, watch } from "vue";

const props = defineProps({
  isPlaying: {
    type: Boolean,
    required: true,
  },
});

const emit = defineEmits(["play", "pause"]);

const isPlaying = ref(props.isPlaying);

watch(
  () => props.isPlaying,
  (newVal) => {
    isPlaying.value = newVal;
  },
);

const togglePlay = () => {
  if (isPlaying.value) {
    emit("pause");
  } else {
    emit("play");
  }
};
</script>
