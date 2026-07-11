<script setup lang="ts">
import { computed, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import type { SortResult } from "../types";

const folderPath = ref<string | null>(null);
const sorting = ref(false);
const result = ref<SortResult | null>(null);
const error = ref<string | null>(null);
const showConfirm = ref(false);

const pickDirName = computed(() => {
  if (!folderPath.value) return "";
  const parts = folderPath.value.split(/[\\/]/).filter(Boolean);
  const base = parts[parts.length - 1] ?? "";
  return base.endsWith("_work") ? base.slice(0, -"_work".length) : base;
});

const confirmMessage = computed(
  () =>
    `評価 1 以上の JPG・RAW・サイドカーを「${pickDirName.value}」フォルダへ移動します。\n元ファイルはこのフォルダから無くなります。続行しますか？`,
);

async function selectFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (typeof selected === "string") {
    folderPath.value = selected;
    result.value = null;
    error.value = null;
  }
}

function requestSort() {
  if (!folderPath.value) return;
  showConfirm.value = true;
}

async function confirmSort() {
  showConfirm.value = false;
  if (!folderPath.value) return;
  sorting.value = true;
  result.value = null;
  error.value = null;
  try {
    result.value = await invoke<SortResult>("sort_photos", { folder: folderPath.value });
  } catch (e) {
    error.value = `ピックアップエラー: ${e}`;
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
        <p class="font-semibold">ピックアップの動作</p>
        <ul class="list-disc list-inside space-y-0.5 text-blue-700">
          <li>評価 1 以上の JPG・対応する RAW・サイドカーを <code class="bg-blue-100 px-1 rounded">{YYYYMMDD}_{場所名}/</code> フォルダへ移動</li>
          <li>移動先フォルダは元フォルダと同じ階層に作成されます（元フォルダ名の末尾が <code class="bg-blue-100 px-1 rounded">_work</code> の場合は除去した名前を使用）</li>
          <li>元ファイルは<span class="font-semibold">移動</span>されます（コピーではありません）</li>
        </ul>
      </div>

      <!-- 実行ボタン -->
      <button
        class="px-4 py-2 text-sm font-medium bg-green-600 text-white rounded hover:bg-green-700 transition-colors disabled:opacity-40"
        :disabled="!folderPath || sorting"
        @click="requestSort"
      >
        {{ sorting ? "ピックアップ中..." : "ピックアップを実行" }}
      </button>

      <!-- エラー -->
      <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded text-red-700 text-sm">
        {{ error }}
      </div>

      <!-- 結果サマリー -->
      <div v-if="result" class="space-y-2">
        <p class="text-sm font-semibold text-gray-700">
          ピックアップ完了 — 移動: {{ result.moved_count }} 件 / スキップ: {{ result.skipped_count }} 件
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

    <!-- ピックアップ確認ダイアログ -->
    <ConfirmDialog
      v-if="showConfirm"
      title="ピックアップ確認"
      :message="confirmMessage"
      @confirm="confirmSort"
      @cancel="showConfirm = false"
    />
  </div>
</template>
