import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { OrganizerResult } from "../types";

export const useOrganizeStore = defineStore("organize", () => {
  const folderPath = ref<string | null>(null);
  const dates = ref<string[]>([]);
  const datePlaceMap = ref<Record<string, string>>({});

  async function scanDates(path: string) {
    folderPath.value = path;
    dates.value = await invoke<string[]>("scan_dates", { folder: path });
    datePlaceMap.value = Object.fromEntries(dates.value.map((d) => [d, ""]));
  }

  async function organizePhotos(): Promise<OrganizerResult> {
    if (!folderPath.value) throw new Error("フォルダが選択されていません");
    return invoke<OrganizerResult>("organize_photos", {
      folder: folderPath.value,
      datePlaceMap: datePlaceMap.value,
    });
  }

  return { folderPath, dates, datePlaceMap, scanDates, organizePhotos };
});
