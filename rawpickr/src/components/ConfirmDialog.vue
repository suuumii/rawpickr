<script setup lang="ts">
defineProps<{
  title: string;
  message: string;
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();
</script>

<template>
  <Teleport to="body">
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="emit('cancel')">
      <div class="bg-white rounded-lg shadow-xl p-6 w-96 max-w-full mx-4">
        <h2 class="text-lg font-semibold text-gray-800 mb-3">{{ title }}</h2>
        <p class="text-gray-600 mb-6 whitespace-pre-wrap">{{ message }}</p>
        <div class="flex justify-end gap-3">
          <button
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors"
            @click="emit('cancel')"
          >
            キャンセル
          </button>
          <button
            class="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 transition-colors"
            @click="emit('confirm')"
          >
            実行
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
