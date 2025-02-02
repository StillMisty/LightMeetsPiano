<template>
  <div
    class="flex h-full max-w-48 min-w-3/5 flex-col justify-center gap-1 rounded-lg border-2 border-slate-400 p-3"
  >
    <h1 class="truncate text-center font-bold">{{ props?.name }}</h1>
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
