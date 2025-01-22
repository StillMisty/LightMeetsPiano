<template>
  <label
    class="cursor-pointer border-slate-400 border-2 rounded-lg p-3 transition hover:bg-slate-50 hover:text-gray-800"
  >
    <input type="file" @change="handleFile" class="hidden" />
    <p><slot></slot></p>
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

<style scoped>
/* input {
  display: none;
}
label {
  padding: 12px 28px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s;

  &:hover {
    background-color: #ffffff;
    color: #000000;
  }
} */
</style>
