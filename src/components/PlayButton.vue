<template>
  <Button @click="togglePlay">
    {{ isPlaying ? "暂停" : "播放" }}
  </Button>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
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
