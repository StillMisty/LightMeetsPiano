<template>
  <div
    class="flex size-full flex-col justify-center gap-1 rounded-md border-2 border-slate-400 p-2 overflow-hidden"
  >
    <h1 class="px-1 w-full truncate text-center font-bold">
      {{ props?.name }}
    </h1>
    <div class="w-full min-w-0 text-left px-1 *:truncate">
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
