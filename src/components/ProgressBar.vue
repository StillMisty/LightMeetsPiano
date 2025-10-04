<template>
  <div class="flex items-center justify-center gap-2 w-full">
    <Slider
      v-model="sliderValue"
      :max="props.duration"
      :step="1"
      class="w-full"
    />
    <span class="text-sm text-muted-foreground select-none"
      >{{ formatTime(props.currentTime) }}/{{
        formatTime(props.duration)
      }}</span
    >
  </div>
</template>

<script lang="ts" setup>
import { Slider } from "@/components/ui/slider";
import { computed } from "vue";

const props = defineProps<{
  duration: number;
  currentTime: number;
}>();

const emit = defineEmits<{
  (e: "updateCurrentTime", value: number): void;
}>();

// 创建一个计算属性来桥接 props 和 Slider 的 v-model
// Slider 的 v-model 需要一个数组，而我们的 prop 是一个数字
const sliderValue = computed({
  // 当组件读取值时，返回一个包含当前时间的数组
  get() {
    return [props.currentTime];
  },
  // 当用户拖动滑块时，Slider 会更新值
  set(newValue: number[]) {
    // 我们获取数组中的第一个值，并通过 emit 事件通知父组件更新
    emit("updateCurrentTime", newValue[0]);
  },
});

const formatTime = (time: number) => {
  // 增加一个保护，防止传入 NaN 或 undefined
  if (isNaN(time) || !isFinite(time)) {
    return "0:00";
  }
  const minutes = Math.floor(time / 60);
  const seconds = Math.floor(time % 60);
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
};
</script>
