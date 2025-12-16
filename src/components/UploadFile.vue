<template>
  <Button
    variant="outline"
    @click="triggerFileInput"
    class="cursor-pointer border-2 border-slate-400!"
  >
    <slot></slot>
    <input
      type="file"
      @change="handleFileChange"
      class="hidden"
      ref="fileInput"
    />
  </Button>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { ref } from "vue";

const emit = defineEmits(["contentChanged"]);
const fileInput = ref<HTMLInputElement | null>(null);

const triggerFileInput = () => {
  if (fileInput.value) {
    fileInput.value.click();
  }
};

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    const file = target.files[0];
    const reader = new FileReader();

    reader.onload = (e: ProgressEvent<FileReader>) => {
      const content = e.target?.result as string;
      emit("contentChanged", content);
    };

    reader.readAsText(file);
  }
};
</script>
