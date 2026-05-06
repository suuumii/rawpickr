<script setup lang="ts">
import { ref } from "vue";
import BrowseView from "./views/BrowseView.vue";
import OrganizeView from "./views/OrganizeView.vue";
import SortView from "./views/SortView.vue";

type Tab = "organize" | "browse" | "sort";
const activeTab = ref<Tab>("browse");

const tabs: { id: Tab; label: string }[] = [
  { id: "organize", label: "撮影日でフォルダ分割" },
  { id: "browse", label: "プレビュー・レーティング" },
  { id: "sort", label: "ピックアップ" },
];
</script>

<template>
  <div class="flex flex-col h-screen bg-gray-100 overflow-hidden">
    <!-- タブバー -->
    <div class="flex-shrink-0 flex border-b border-gray-300 bg-white">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="px-6 py-2.5 text-sm font-medium transition-colors border-b-2 -mb-px"
        :class="activeTab === tab.id
          ? 'border-blue-600 text-blue-600 bg-white'
          : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
        @click="activeTab = tab.id"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- タブコンテンツ -->
    <div class="flex-1 overflow-hidden relative">
      <BrowseView v-show="activeTab === 'browse'" class="absolute inset-0" />
      <OrganizeView v-show="activeTab === 'organize'" class="absolute inset-0" />
      <SortView v-show="activeTab === 'sort'" class="absolute inset-0" />
    </div>
  </div>
</template>
