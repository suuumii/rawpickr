import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { Photo, ExifInfo } from "../types";

export const useBrowseStore = defineStore("browse", () => {
  const folderPath = ref<string | null>(null);
  const photos = ref<Photo[]>([]);
  const currentIndex = ref<number | null>(null);
  const currentExif = ref<ExifInfo | null>(null);
  const currentPreviewUrl = ref<string | null>(null);

  const currentPhoto = () =>
    currentIndex.value !== null ? photos.value[currentIndex.value] ?? null : null;

  async function loadFolder(path: string) {
    folderPath.value = path;
    photos.value = await invoke<Photo[]>("list_photos", { folder: path });
    currentIndex.value = null;
    currentExif.value = null;
    currentPreviewUrl.value = null;
  }

  async function selectPhoto(index: number) {
    if (index < 0 || index >= photos.value.length) return;
    currentIndex.value = index;
    const photo = photos.value[index];

    // プレビュー URL を解決
    if (photo.file_type === "raw") {
      try {
        currentPreviewUrl.value = await invoke<string>("read_raw_preview", { path: photo.path });
      } catch {
        currentPreviewUrl.value = null;
      }
    } else {
      currentPreviewUrl.value = convertFileSrc(photo.path);
    }

    // EXIF を取得（RAW も試みる）
    currentExif.value = await invoke<ExifInfo>("read_exif", { path: photo.path });
  }

  async function setRating(rating: number) {
    const photo = currentPhoto();
    if (!photo || folderPath.value === null) return;
    await invoke("write_rating", {
      folder: folderPath.value,
      filename: photo.filename,
      rating,
    });
    photos.value[currentIndex.value!] = { ...photo, rating };
  }

  async function removePhoto(index: number) {
    photos.value.splice(index, 1);
    if (photos.value.length === 0) {
      currentIndex.value = null;
      currentExif.value = null;
      currentPreviewUrl.value = null;
    } else {
      const next = Math.min(index, photos.value.length - 1);
      await selectPhoto(next);
    }
  }

  // RAW のみ削除後: file_type を "jpg" に更新してプレビューを再解決
  async function downgradeToJpg(index: number) {
    const photo = photos.value[index];
    if (!photo) return;
    photos.value[index] = { ...photo, file_type: "jpg" };
    await selectPhoto(index);
  }

  function navigate(delta: number) {
    if (photos.value.length === 0) return;
    const base = currentIndex.value ?? 0;
    const next = Math.max(0, Math.min(photos.value.length - 1, base + delta));
    selectPhoto(next);
  }

  return {
    folderPath,
    photos,
    currentIndex,
    currentExif,
    currentPreviewUrl,
    currentPhoto,
    loadFolder,
    selectPhoto,
    setRating,
    removePhoto,
    downgradeToJpg,
    navigate,
  };
});
