<template>
  <div
    class="z-10 flex h-36 cursor-pointer flex-col items-center justify-center gap-4 rounded-lg border-2 border-dashed border-slate-400 p-8 transition-colors"
    :class="{ 'bg-blue-50/30': isDragging }"
    @dragover.prevent="isDragging = true"
    @dragleave.prevent="isDragging = false"
    @drop.prevent.stop="handleDrop"
  >
    <Upload :size="32" />
    <slot></slot>

    <input
      type="file"
      class="hidden"
      ref="fileInput"
      accept=".txt"
      @change="handleFileInput"
    />
  </div>
</template>

<script lang="ts" setup>
import { Upload } from "lucide-vue-next";
import { ref } from "vue";

const emit = defineEmits(["file-dropped"]);
const isDragging = ref(false);
const fileInput = ref<HTMLInputElement | null>(null);

defineExpose({ fileInput });

function handleDrop(e: DragEvent) {
  isDragging.value = false;

  const files = e.dataTransfer?.files;
  if (!files || files.length === 0) return;

  const file = files[0];
  if (file.name.endsWith(".txt")) {
    processFile(file);
  } else {
    emit("file-dropped", "非合法 TXT 谱，重新选择");
  }
}

function handleFileInput(e: Event) {
  const target = e.target as HTMLInputElement;
  const files = target.files;

  if (files && files.length > 0) {
    const file = files[0];
    processFile(file);
    // 清空 input，确保同一文件可以再次选择
    target.value = "";
  }
}

function processFile(file: File) {
  const reader = new FileReader();

  reader.onload = (event) => {
    console.log("File content:", event.target?.result);

    if (event.target && typeof event.target.result === "string") {
      emit("file-dropped", event.target.result);
    }
  };

  reader.readAsText(file);
}
</script>
