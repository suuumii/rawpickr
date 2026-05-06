<script setup lang="ts">
import { ref, onUnmounted } from "vue";

const MIN_WIDTH = 150;

const containerRef = ref<HTMLElement | null>(null);
const leftWidth = ref(380);
const dragging = ref(false);

function onDividerMousedown(e: MouseEvent) {
  e.preventDefault();
  dragging.value = true;
  document.addEventListener("mousemove", onMousemove);
  document.addEventListener("mouseup", onMouseup);
}

function onMousemove(e: MouseEvent) {
  if (!dragging.value || !containerRef.value) return;
  const rect = containerRef.value.getBoundingClientRect();
  const newLeft = e.clientX - rect.left;
  const maxLeft = rect.width - MIN_WIDTH - 6;
  leftWidth.value = Math.max(MIN_WIDTH, Math.min(newLeft, maxLeft));
}

function onMouseup() {
  dragging.value = false;
  document.removeEventListener("mousemove", onMousemove);
  document.removeEventListener("mouseup", onMouseup);
}

onUnmounted(() => {
  document.removeEventListener("mousemove", onMousemove);
  document.removeEventListener("mouseup", onMouseup);
});
</script>

<template>
  <div ref="containerRef" class="flex h-full w-full overflow-hidden select-none">
    <!-- 左ペイン -->
    <div :style="{ width: leftWidth + 'px', minWidth: MIN_WIDTH + 'px' }" class="flex-shrink-0 overflow-hidden">
      <slot name="left" />
    </div>

    <!-- スプリッターバー -->
    <div
      class="w-1.5 flex-shrink-0 bg-gray-300 hover:bg-blue-400 transition-colors duration-150 cursor-col-resize"
      :class="{ 'bg-blue-400': dragging }"
      @mousedown="onDividerMousedown"
    />

    <!-- 右ペイン -->
    <div class="flex-1 overflow-hidden min-w-0">
      <slot name="right" />
    </div>
  </div>
</template>
