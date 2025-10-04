<template>
  <div
    data-tauri-drag-region
    class="flex justify-end border-b border-gray-600 text-gray-500 *:grid *:size-8 *:cursor-pointer *:place-items-center *:transition-colors"
  >
    <Button variant="ghost" @click="minimizeWindow" title="最小化">
      <Minus />
    </Button>
    <Button variant="ghost" @click="closeWindow" title="关闭">
      <X />
    </Button>
  </div>
</template>

<script lang="ts" setup>
import { Button } from "@/components/ui/button";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-vue-next";

const minimizeWindow = async () => {
  await getCurrentWindow().minimize();
};

const closeWindow = async () => {
  await getCurrentWindow().close();
};
</script>

<style scoped>
[data-tauri-drag-region] {
  -webkit-app-region: drag;

  & button {
    /* 确保拖动区域内的可交互元素不会触发拖动 */
    -webkit-app-region: no-drag;
  }
}
</style>
