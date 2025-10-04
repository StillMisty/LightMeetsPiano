<template>
  <div>
    <Input
      type="file"
      @change="handleFileChange"
      class="hidden"
      ref="fileInput"
    />
    <Button variant="outline" @click="triggerFileInput">
      {{ props.msg }}
    </Button>
  </div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ref } from "vue";

const props = defineProps({
  msg: {
    type: String,
    default: "选择文件",
  },
});

const emit = defineEmits(["contentChanged"]);
const fileInput = ref<HTMLInputElement | null>(null);

const triggerFileInput = () => {
  fileInput.value?.click();
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
