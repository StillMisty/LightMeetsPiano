<template>
  <div class="container flex items-center justify-center gap-2">
    <Progress :value="progress" :max="100" class="w-64" />
    <span class="cursor-default"
      >{{ formatTime(props.currentTime) }} /
      {{ formatTime(props.duration) }}</span
    >
  </div>
</template>

<script lang="ts" setup>
import { Progress } from "@/components/ui/progress";
import { computed } from "vue";

const props = defineProps<{
  duration: number;
  currentTime: number;
}>();
const emit = defineEmits(["updateCurrentTime"]);

const progress = computed(() => {
  if (props.duration === 0) return 0;
  return (props.currentTime / props.duration) * 100;
});

const formatTime = (time: number) => {
  const minutes = Math.floor(time / 60);
  const seconds = Math.floor(time % 60);
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
};
</script>
