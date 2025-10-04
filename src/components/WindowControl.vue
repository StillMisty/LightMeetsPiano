<template>
  <div
    class="window-drag-area flex h-8 justify-end border-b border-gray-600 text-gray-500 *:grid *:size-8 *:cursor-pointer *:place-items-center *:transition-colors"
  >
    <Button
      variant="ghost"
      class="hover:bg-gray-700"
      @click="minimizeWindow"
      title="最小化"
    >
      <Minus />
    </Button>
    <Button
      variant="ghost"
      class="hover:bg-red-600"
      @click="closeWindow"
      title="关闭"
    >
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
.window-drag-area {
  -webkit-app-region: drag; /* 允许拖动 */

  button {
    -webkit-app-region: no-drag; /* 禁止拖动 */
  }
}
</style>
