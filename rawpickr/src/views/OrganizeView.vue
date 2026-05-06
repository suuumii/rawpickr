<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import type { OrganizerResult } from "../types";
import { useOrganizeStore } from "../stores/organize";

const store = useOrganizeStore();
const scanning = ref(false);
const organizing = ref(false);
const result = ref<OrganizerResult | null>(null);
const error = ref<string | null>(null);

async function selectFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (typeof selected === "string") {
    result.value = null;
    error.value = null;
    scanning.value = true;
    try {
      await store.scanDates(selected);
    } catch (e) {
      error.value = `スキャンエラー: ${e}`;
    } finally {
      scanning.value = false;
    }
  }
}

async function runOrganize() {
  organizing.value = true;
  result.value = null;
  error.value = null;
  try {
    result.value = await store.organizePhotos();
  } catch (e) {
    error.value = `整理エラー: ${e}`;
  } finally {
    organizing.value = false;
  }
}
</script>

<template>
  <div class="h-full flex flex-col overflow-hidden">
    <!-- ツールバー -->
    <div class="flex-shrink-0 flex items-center gap-3 px-3 py-2 bg-gray-50 border-b border-gray-200">
      <button
        class="px-3 py-1.5 text-sm font-medium bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors disabled:opacity-40"
        :disabled="scanning"
        @click="selectFolder"
      >
        📁 フォルダを選択
      </button>
      <span class="text-sm text-gray-500 truncate">
        {{ store.folderPath ?? "フォルダが選択されていません" }}
      </span>
      <span v-if="scanning" class="ml-auto text-sm text-blue-500">スキャン中...</span>
    </div>

    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <!-- 日付・場所入力テーブル -->
      <div v-if="store.dates.length > 0">
        <h2 class="text-sm font-semibold text-gray-700 mb-2">撮影日ごとに場所を入力してください</h2>
        <table class="w-full text-sm border border-gray-200 rounded-lg overflow-hidden">
          <thead class="bg-gray-100 text-gray-600 text-xs uppercase">
            <tr>
              <th class="text-left px-4 py-2 w-32">撮影日</th>
              <th class="text-left px-4 py-2">場所（フォルダ名サフィックス）</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="date in store.dates"
              :key="date"
              class="border-t border-gray-100 hover:bg-gray-50"
            >
              <td class="px-4 py-2 font-mono text-gray-800">{{ date }}</td>
              <td class="px-4 py-2">
                <input
                  v-model="store.datePlaceMap[date]"
                  type="text"
                  placeholder="例: 東京, 大阪"
                  class="w-full px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:border-blue-400"
                />
              </td>
            </tr>
          </tbody>
        </table>

        <button
          class="mt-4 px-4 py-2 text-sm font-medium bg-green-600 text-white rounded hover:bg-green-700 transition-colors disabled:opacity-40"
          :disabled="organizing"
          @click="runOrganize"
        >
          {{ organizing ? "整理中..." : "整理を実行" }}
        </button>
      </div>

      <div v-else-if="!scanning && store.folderPath" class="text-gray-400 text-sm">
        JPG ファイルが見つかりませんでした。
      </div>

      <!-- エラー -->
      <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded text-red-700 text-sm">
        {{ error }}
      </div>

      <!-- 結果ログ -->
      <div v-if="result" class="space-y-2">
        <p class="text-sm font-semibold text-gray-700">
          整理完了 — フォルダ: {{ result.folder_count }} 件 / 移動: {{ result.moved_count }} 件 / スキップ: {{ result.skipped_count }} 件
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
