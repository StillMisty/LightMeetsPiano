<template>
  <div
    class="flex flex-col justify-center gap-1 p-3 border-slate-400 border-2 rounded-lg min-w-3/5 max-w-48 h-full"
  >
    <h1 class="text-center font-bold truncate">{{ props?.name }}</h1>
    <div class="text-left *:truncate">
      <p>作者：{{ props?.author || "佚名" }}</p>
      <p>谱者：{{ props?.transcribedBy || "佚名" }}</p>
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  name: string; // 曲谱名称
  author: string; // 作者
  transcribedBy: string; // 谱曲者
}>();
</script>
