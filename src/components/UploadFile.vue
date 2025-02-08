<template>
  <label
    class="cursor-pointer rounded-lg border-2 border-slate-400 p-3 text-nowrap transition hover:bg-slate-50 hover:text-gray-800"
  >
    <input type="file" @change="handleFile" class="hidden" accept=".txt" />
    <slot></slot>
  </label>
</template>

<script lang="ts" setup>
const emit = defineEmits(["contentChanged"]);

function handleFile(e: Event) {
  const target = e.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = (evt) => {
      if (evt.target) {
        emit("contentChanged", evt.target.result as string);
      }
    };
    reader.readAsText(file);
  }
}
</script>
