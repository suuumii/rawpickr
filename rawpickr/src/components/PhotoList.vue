<script setup lang="ts">
import type { Photo } from "../types";

defineProps<{
  photos: Photo[];
  selectedIndex: number | null;
}>();

const emit = defineEmits<{
  select: [index: number];
}>();

const STARS: Record<number, string> = {
  0: "☆☆☆☆☆",
  1: "★☆☆☆☆",
  2: "★★☆☆☆",
  3: "★★★☆☆",
  4: "★★★★☆",
  5: "★★★★★",
};
</script>

<template>
  <div class="h-full flex flex-col bg-white">
    <div class="flex-shrink-0 grid grid-cols-[1fr_auto] px-3 py-1.5 bg-gray-100 border-b border-gray-200 text-xs font-medium text-gray-500 uppercase tracking-wide">
      <span>ファイル名</span>
      <span>評価</span>
    </div>

    <div class="flex-1 overflow-y-auto">
      <div
        v-for="(photo, i) in photos"
        :key="photo.path"
        class="grid grid-cols-[1fr_auto] px-3 py-1.5 cursor-pointer border-b border-gray-100 text-sm hover:bg-blue-50 transition-colors"
        :class="{ 'bg-blue-100 font-medium': i === selectedIndex }"
        @click="emit('select', i)"
      >
        <span class="flex items-center gap-1.5 min-w-0">
          <span class="truncate text-gray-800">{{ photo.filename }}</span>
          <span
            v-if="photo.file_type === 'jpg'"
            class="flex-shrink-0 px-1 py-0.5 text-[10px] font-medium rounded bg-blue-100 text-blue-600"
          >JPG</span>
          <span
            v-else-if="photo.file_type === 'raw'"
            class="flex-shrink-0 px-1 py-0.5 text-[10px] font-medium rounded bg-orange-100 text-orange-600"
          >RAW</span>
        </span>
        <span class="text-yellow-500 text-xs ml-2">{{ STARS[photo.rating] ?? STARS[0] }}</span>
      </div>

      <div v-if="photos.length === 0" class="flex items-center justify-center h-24 text-gray-400 text-sm">
        フォルダを選択してください
      </div>
    </div>
  </div>
</template>
