<script setup lang="ts">
defineProps<{
  previewUrl: string | null;
}>();
</script>

<template>
  <div class="h-full flex items-center justify-center bg-gray-900 overflow-hidden">
    <img
      v-if="previewUrl"
      :src="previewUrl"
      :key="previewUrl"
      class="max-w-full max-h-full object-contain"
      alt="preview"
    />
    <span v-else class="text-gray-500 text-sm">写真を選択してください</span>
  </div>
</template>
