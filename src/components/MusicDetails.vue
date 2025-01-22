<template>
  <div class="flex flex-col gap-1 text-center p-3 border-slate-400 border-2 rounded-lg">
    <h1>{{ props?.name }}</h1>
    <p>作者：{{ props?.author }}</p>
    <p>谱曲者：{{ props?.transcribedBy }}</p>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  name: string; // 曲谱名称
  author: string; // 作者
  transcribedBy: string; // 谱曲者
}>();
</script>
