<template>
  <div>
    <input
      type="file"
      @change="handleFileChange"
      class="hidden"
      ref="fileInput"
    />
    <Button
      variant="secondary"
      @click="triggerFileInput"
      class="cursor-pointer"
    >
      <slot></slot>
    </Button>
  </div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { ref } from "vue";

const emit = defineEmits(["contentChanged"]);
const fileInput = ref<HTMLInputElement | null>(null);

const triggerFileInput = () => {
  console.log("triggerFileInput called");
  if (fileInput.value) {
    fileInput.value.click();
    console.log("fileInput clicked");
  } else {
    console.error("fileInput is null");
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
