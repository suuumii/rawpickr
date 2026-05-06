<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { SortResult } from "../types";

const folderPath = ref<string | null>(null);
const sorting = ref(false);
const result = ref<SortResult | null>(null);
const error = ref<string | null>(null);

async function selectFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (typeof selected === "string") {
    folderPath.value = selected;
    result.value = null;
    error.value = null;
  }
}

async function runSort() {
  if (!folderPath.value) return;
  sorting.value = true;
  result.value = null;
  error.value = null;
  try {
    result.value = await invoke<SortResult>("sort_photos", { folder: folderPath.value });
  } catch (e) {
    error.value = `仕分けエラー: ${e}`;
  } finally {
    sorting.value = false;
  }
}
</script>

<template>
  <div class="h-full flex flex-col overflow-hidden">
    <!-- ツールバー -->
    <div class="flex-shrink-0 flex items-center gap-3 px-3 py-2 bg-gray-50 border-b border-gray-200">
      <button
        class="px-3 py-1.5 text-sm font-medium bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
        @click="selectFolder"
      >
        📁 フォルダを選択
      </button>
      <span class="text-sm text-gray-500 truncate">
        {{ folderPath ?? "フォルダが選択されていません" }}
      </span>
    </div>

    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <!-- 説明 -->
      <div class="p-4 bg-blue-50 border border-blue-200 rounded-lg text-sm text-blue-800 space-y-1">
        <p class="font-semibold">仕分けの動作</p>
        <ul class="list-disc list-inside space-y-0.5 text-blue-700">
          <li>評価 1 以上の JPG を <code class="bg-blue-100 px-1 rounded">_pick/</code> フォルダへコピー</li>
          <li>対応する RAW ファイルを <code class="bg-blue-100 px-1 rounded">_raw_pick/</code> フォルダへコピー</li>
          <li>元ファイルは削除しません</li>
        </ul>
      </div>

      <!-- 実行ボタン -->
      <button
        class="px-4 py-2 text-sm font-medium bg-green-600 text-white rounded hover:bg-green-700 transition-colors disabled:opacity-40"
        :disabled="!folderPath || sorting"
        @click="runSort"
      >
        {{ sorting ? "仕分け中..." : "仕分けを実行" }}
      </button>

      <!-- エラー -->
      <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded text-red-700 text-sm">
        {{ error }}
      </div>

      <!-- 結果サマリー -->
      <div v-if="result" class="space-y-2">
        <p class="text-sm font-semibold text-gray-700">
          仕分け完了 — コピー: {{ result.copied_count }} 件 / スキップ: {{ result.skipped_count }} 件
        </p>
        <div class="bg-gray-900 rounded p-3 max-h-64 overflow-y-auto">
          <p
            v-for="(line, i) in result.logs"
            :key="i"
            class="text-xs text-gray-300 font-mono"
          >{{ line }}</p>
        </div>
      </div>
    </div>
  </div>
</template>
