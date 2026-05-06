<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import SplitPane from "../components/SplitPane.vue";
import PhotoList from "../components/PhotoList.vue";
import PhotoPreview from "../components/PhotoPreview.vue";
import ConfirmDialog from "../components/ConfirmDialog.vue";
import { useBrowseStore } from "../stores/browse";
import type { DeleteMode, ExifInfo } from "../types";

const store = useBrowseStore();

const STARS: Record<number, string> = {
  0: "☆☆☆☆☆",
  1: "★☆☆☆☆",
  2: "★★☆☆☆",
  3: "★★★☆☆",
  4: "★★★★☆",
  5: "★★★★★",
};

const exifParts = computed(() => {
  const e: ExifInfo | null = store.currentExif;
  if (!e) return [];
  return [
    e.camera && `📷 ${e.camera}`,
    e.lens && `🔭 ${e.lens}`,
    e.f_number && `F${e.f_number.toFixed(1)}`,
    e.shutter_speed && e.shutter_speed,
    e.iso && `ISO ${e.iso}`,
    e.focal_length && `${Math.round(e.focal_length)}mm`,
    e.taken_at && e.taken_at,
  ].filter(Boolean) as string[];
});

// 削除確認ダイアログの状態
const deleteDialog = ref<{ visible: boolean; mode: DeleteMode; message: string }>({
  visible: false,
  mode: "both",
  message: "",
});

// 削除対象なし通知ダイアログの状態
const infoDialog = ref<{ visible: boolean; message: string }>({
  visible: false,
  message: "",
});

async function selectFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (typeof selected === "string") {
    await store.loadFolder(selected);
  }
}

function showDeleteDialog(mode: DeleteMode) {
  const photo = store.currentPhoto();
  if (!photo) return;

  // JPG のみファイルに「RAW のみ削除」は対象なし
  if (photo.file_type === "jpg" && mode === "raw_only") {
    infoDialog.value = {
      visible: true,
      message: "RAW ファイルが存在しないため、RAW のみ削除はできません。",
    };
    return;
  }

  // RAW のみファイルに「JPG+RAW 削除」は RAW だけ削除
  let label: string;
  if (photo.file_type === "raw") {
    label = "RAW ファイルを";
  } else if (mode === "both") {
    label = "JPG と RAW を両方";
  } else {
    label = "RAW のみ";
  }

  deleteDialog.value = {
    visible: true,
    mode,
    message: `${label}削除しますか？\n${photo.filename}`,
  };
}

async function confirmDelete() {
  const photo = store.currentPhoto();
  const idx = store.currentIndex;
  if (!photo || idx === null) return;
  deleteDialog.value.visible = false;
  const mode = deleteDialog.value.mode;
  try {
    await invoke("delete_photo", { path: photo.path, mode });
    if (mode === "raw_only" && photo.file_type === "both") {
      // RAW を削除したが JPG は残るので、JPG のみとして一覧を更新
      await store.downgradeToJpg(idx);
    } else {
      await store.removePhoto(idx);
    }
  } catch (e) {
    infoDialog.value = { visible: true, message: `削除に失敗しました: ${e}` };
  }
}

// キーボードショートカット
function onKeydown(e: KeyboardEvent) {
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

  switch (e.key) {
    case "1": case "2": case "3": case "4": case "5":
      store.setRating(Number(e.key));
      break;
    case "0":
      store.setRating(0);
      break;
    case "ArrowLeft":
      e.preventDefault();
      store.navigate(-1);
      break;
    case "ArrowRight":
      e.preventDefault();
      store.navigate(1);
      break;
    case "Delete":
      showDeleteDialog("both");
      break;
  }
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- ツールバー -->
    <div class="flex-shrink-0 flex items-center gap-3 px-3 py-2 bg-gray-50 border-b border-gray-200">
      <button
        class="px-3 py-1.5 text-sm font-medium bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
        @click="selectFolder"
      >
        📁 フォルダを選択
      </button>
      <span class="text-sm text-gray-500 truncate">
        {{ store.folderPath ?? "フォルダが選択されていません" }}
      </span>
      <span class="ml-auto text-sm text-gray-400">{{ store.photos.length }} 枚</span>
    </div>

    <!-- スプリッター -->
    <div class="flex-1 overflow-hidden">
      <SplitPane>
        <template #left>
          <PhotoList
            :photos="store.photos"
            :selected-index="store.currentIndex"
            @select="store.selectPhoto"
          />
        </template>
        <template #right>
          <div class="h-full flex flex-col">
            <!-- 画像エリア -->
            <div class="flex-1 overflow-hidden">
              <PhotoPreview :preview-url="store.currentPreviewUrl" />
            </div>

            <!-- 下部グリッドバー: 左=評価・EXIF / 右=削除ボタン -->
            <div class="flex-shrink-0 grid grid-cols-[1fr_auto] gap-x-4 items-center px-3 py-2 bg-gray-800 border-t border-gray-700">
              <!-- 左列: 評価・ファイル名・EXIF -->
              <div class="min-w-0 space-y-1">
                <div v-if="store.currentPhoto()" class="flex items-center gap-3">
                  <span class="text-yellow-400 text-base leading-none">
                    {{ STARS[store.currentPhoto()?.rating ?? 0] ?? STARS[0] }}
                  </span>
                  <span class="text-gray-300 text-xs truncate">
                    {{ store.currentPhoto()?.filename }}
                  </span>
                </div>
                <div v-if="exifParts.length" class="flex flex-wrap gap-x-3 gap-y-0.5">
                  <span
                    v-for="part in exifParts"
                    :key="part"
                    class="text-gray-400 text-xs"
                  >{{ part }}</span>
                </div>
                <div v-else-if="store.currentPhoto()" class="text-gray-600 text-xs">
                  EXIF なし
                </div>
                <div v-else class="text-gray-600 text-xs">
                  キー: 1–5 評価　0 リセット　← → 移動　Del 削除
                </div>
              </div>

              <!-- 右列: 削除ボタン -->
              <div class="flex flex-col gap-1.5 flex-shrink-0">
                <button
                  class="px-3 py-1 text-xs font-medium text-white bg-red-700 rounded hover:bg-red-800 transition-colors disabled:opacity-40"
                  :disabled="!store.currentPhoto()"
                  @click="showDeleteDialog('both')"
                >
                  JPG+RAW 削除
                </button>
                <button
                  class="px-3 py-1 text-xs font-medium text-white bg-orange-700 rounded hover:bg-orange-800 transition-colors disabled:opacity-40"
                  :disabled="!store.currentPhoto()"
                  @click="showDeleteDialog('raw_only')"
                >
                  RAW のみ削除
                </button>
              </div>
            </div>
          </div>
        </template>
      </SplitPane>
    </div>

    <!-- 削除確認ダイアログ -->
    <ConfirmDialog
      v-if="deleteDialog.visible"
      title="削除確認"
      :message="deleteDialog.message"
      @confirm="confirmDelete"
      @cancel="deleteDialog.visible = false"
    />

    <!-- 削除対象なし / エラー通知ダイアログ -->
    <Teleport to="body">
      <div
        v-if="infoDialog.visible"
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
        @click.self="infoDialog.visible = false"
      >
        <div class="bg-white rounded-lg shadow-xl p-6 w-96 max-w-full mx-4">
          <h2 class="text-lg font-semibold text-gray-800 mb-3">削除できません</h2>
          <p class="text-gray-600 mb-6 whitespace-pre-wrap">{{ infoDialog.message }}</p>
          <div class="flex justify-end">
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 transition-colors"
              @click="infoDialog.visible = false"
            >
              OK
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
