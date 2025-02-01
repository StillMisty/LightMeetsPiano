<template>
  <div
    class="window-drag-area flex justify-end h-8 border-b text-gray-500 border-gray-600 *:w-8 *:h-8 *:transition-colors *:grid *:place-items-center"
  >
    <button class="hover:bg-gray-700" @click="minimizeWindow" title="最小化">
      <svg
        t="1738394555840"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        p-id="1155"
        width="20"
        height="20"
      >
        <path
          d="M65.23884 456.152041 958.760137 456.152041l0 111.695918L65.23884 567.847959 65.23884 456.152041z"
          fill="currentColor"
          p-id="1156"
        ></path>
      </svg>
    </button>
    <button class="hover:bg-red-600" @click="closeWindow" title="关闭">
      <svg
        t="1738394573895"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        p-id="1350"
        width="20"
        height="20"
      >
        <path
          d="M956.171172 875.411847l-80.757279 80.757279L511.997953 592.757279 148.586107 956.170149 67.828828 875.411847l363.411847-363.411847L67.828828 148.58713l80.757279-80.757279 363.411847 363.411847L875.413893 67.829851l80.757279 80.757279L592.756255 512 956.171172 875.411847z"
          fill="currentColor"
          p-id="1351"
        ></path>
      </svg>
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
