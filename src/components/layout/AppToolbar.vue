<script setup lang="ts">
import { ref } from 'vue'
import type { ViewMode } from '../../types'

defineProps<{
  currentView: ViewMode
  searchQuery: string
}>()

const emit = defineEmits<{
  'update:currentView': [mode: ViewMode]
  'update:searchQuery': [query: string]
  'submit-url': [url: string]
}>()

const urlInput = ref('')
const showSearch = ref(false)

function handleSubmitUrl() {
  const url = urlInput.value.trim()
  if (url) {
    emit('submit-url', url)
    urlInput.value = ''
  }
}

function handleUrlKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') handleSubmitUrl()
}

function toggleSearch() {
  showSearch.value = !showSearch.value
  if (!showSearch.value) {
    emit('update:searchQuery', '')
  }
}

function handleSearchInput(e: Event) {
  emit('update:searchQuery', (e.target as HTMLInputElement).value)
}
</script>

<template>
  <header
    class="h-[var(--toolbar-height)] flex items-center px-4 gap-3 border-b border-[var(--color-separator)] bg-white/80 dark:bg-neutral-900/80 backdrop-blur-xl"
    data-tauri-drag-region
  >
    <!-- Spacer for traffic lights -->
    <div class="w-16 flex-shrink-0" data-tauri-drag-region />

    <!-- URL input or Search -->
    <template v-if="showSearch">
      <input
        type="text"
        :value="searchQuery"
        @input="handleSearchInput"
        placeholder="検索..."
        class="flex-1 h-8 px-3 rounded-md bg-neutral-100 dark:bg-neutral-800 text-sm outline-none focus:ring-2 focus:ring-[var(--color-accent)]"
        autofocus
      />
    </template>
    <template v-else>
      <input
        v-model="urlInput"
        type="url"
        placeholder="URLを入力..."
        class="flex-1 h-8 px-3 rounded-md bg-neutral-100 dark:bg-neutral-800 text-sm outline-none focus:ring-2 focus:ring-[var(--color-accent)]"
        @keydown="handleUrlKeydown"
      />
    </template>

    <!-- Search toggle -->
    <button class="w-8 h-8 flex items-center justify-center rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-800 text-neutral-500"
            :class="{ 'text-[var(--color-accent)]': showSearch }"
            @click="toggleSearch"
            title="検索">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
    </button>

    <!-- Download button -->
    <button v-if="!showSearch"
            class="px-4 h-8 rounded-md bg-[var(--color-accent)] text-white text-sm font-medium flex-shrink-0"
            @click="handleSubmitUrl">
      ダウンロード
    </button>

    <!-- View mode toggle -->
    <div class="flex gap-0.5 bg-neutral-100 dark:bg-neutral-800 rounded-md p-0.5 flex-shrink-0">
      <button
        v-for="mode in (['list', 'grid', 'column'] as ViewMode[])"
        :key="mode"
        class="px-2 py-1 text-xs rounded"
        :class="currentView === mode ? 'bg-white dark:bg-neutral-700 shadow-sm' : 'text-neutral-500'"
        @click="emit('update:currentView', mode)"
      >
        {{ mode === 'list' ? 'リスト' : mode === 'grid' ? 'グリッド' : 'カラム' }}
      </button>
    </div>
  </header>
</template>
