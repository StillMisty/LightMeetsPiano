<template>
    <div class="container flex items-center justify-center gap-2">
        <input class="w-64 cursor-pointer" type="range" :max="props.duration" :value="props.currentTime" @input="onInput"></input>
        <span class="cursor-default" >{{ formatTime(props.currentTime) }} / {{ formatTime(props.duration) }}</span>
    </div>
    
</template>

<script lang="ts" setup>
const props = defineProps<{
  duration: number;
  currentTime: number;
}>();
const emit = defineEmits(["updateCurrentTime"]);

const onInput=(e: Event) =>{
  emit("updateCurrentTime", parseFloat((e.target as HTMLInputElement).value));
}

const formatTime = (time: number) => {
  const minutes = Math.floor(time / 60);
  const seconds = Math.floor(time % 60);
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
};

</script>
