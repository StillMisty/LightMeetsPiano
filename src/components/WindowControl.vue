<template>
  <div
    class="window-drag-area flex justify-end h-8 border-b border-gray-600 *:w-8 *:h-8 *:transition-colors"
  >
    <button class="hover:bg-gray-700" @click="minimizeWindow" title="最小化">
      <span class="transform translate-y-[-2px]">-</span>
    </button>
    <button class="hover:bg-red-600" @click="closeWindow" title="关闭">
      X
    </button>
  </div>
</template>

<script lang="ts" setup>
import { getCurrentWindow } from "@tauri-apps/api/window";

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
