<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlaylistsStore } from '../../stores/playlists'
import type { UrlList } from '../../types'

defineProps<{
  urlList: UrlList | null
}>()

const emit = defineEmits<{
  'batch-download': [urls: string[]]
}>()

const store = usePlaylistsStore()

const newUrl = ref('')
const importText = ref('')
const showImport = ref(false)

const items = computed(() => store.currentUrlListItems)

function addUrl() {
  const url = newUrl.value.trim()
  if (!url) return
  // TODO: invoke add_url_to_list command
  newUrl.value = ''
}

function handleImport() {
  // TODO: invoke batch add URLs using parsed lines from importText
  // const lines = importText.value.split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('#'))
  importText.value = ''
  showImport.value = false
}

function handleExport() {
  const urls = items.value.map(item => item.url).join('\n')
  // TODO: Use tauri-plugin-dialog to save file
  navigator.clipboard.writeText(urls)
}

function handleBatchDownload() {
  const urls = items.value.map(item => item.url)
  emit('batch-download', urls)
}

async function handleRemoveItem(_itemId: number) {
  // TODO: invoke remove_url_from_list command
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') addUrl()
}
</script>

<template>
  <div v-if="!urlList" class="p-8 text-center text-neutral-400 text-sm">
    URLリストを選択してください
  </div>

  <div v-else>
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-[var(--color-separator)]">
      <div>
        <h2 class="text-lg font-semibold">{{ urlList.name }}</h2>
        <p class="text-xs text-neutral-400 mt-0.5">{{ items.length }} URL</p>
      </div>
      <div class="flex gap-2">
        <button class="px-3 py-1.5 rounded-md text-xs bg-neutral-100 dark:bg-neutral-700"
                @click="showImport = !showImport">
          インポート
        </button>
        <button class="px-3 py-1.5 rounded-md text-xs bg-neutral-100 dark:bg-neutral-700"
                @click="handleExport">
          エクスポート
        </button>
        <button class="px-3 py-1.5 rounded-md text-xs bg-[var(--color-accent)] text-white"
                @click="handleBatchDownload"
                :disabled="items.length === 0">
          一括ダウンロード
        </button>
      </div>
    </div>

    <!-- Import area -->
    <div v-if="showImport" class="px-4 py-3 border-b border-[var(--color-separator)] bg-neutral-50 dark:bg-neutral-900">
      <p class="text-xs text-neutral-500 mb-2">URLを改行区切りで貼り付けてください（#で始まる行はコメント）</p>
      <textarea v-model="importText"
                class="w-full h-24 px-3 py-2 text-xs font-mono rounded-md bg-white dark:bg-neutral-800 border border-[var(--color-separator)] outline-none focus:ring-1 focus:ring-[var(--color-accent)] resize-none"
                placeholder="https://www.youtube.com/watch?v=...&#10;https://www.youtube.com/watch?v=..."></textarea>
      <div class="flex justify-end gap-2 mt-2">
        <button class="px-3 py-1 rounded text-xs bg-neutral-200 dark:bg-neutral-700" @click="showImport = false">
          キャンセル
        </button>
        <button class="px-3 py-1 rounded text-xs bg-[var(--color-accent)] text-white" @click="handleImport">
          追加
        </button>
      </div>
    </div>

    <!-- Add URL input -->
    <div class="flex gap-2 px-4 py-2 border-b border-[var(--color-separator)]">
      <input v-model="newUrl"
             class="flex-1 h-7 px-2 text-xs rounded-md bg-neutral-100 dark:bg-neutral-700 outline-none focus:ring-1 focus:ring-[var(--color-accent)]"
             placeholder="URLを追加..."
             @keydown="handleKeydown" />
      <button class="px-3 h-7 rounded-md text-xs bg-[var(--color-accent)] text-white" @click="addUrl">
        追加
      </button>
    </div>

    <!-- URL list -->
    <div v-if="items.length === 0" class="p-8 text-center text-neutral-400 text-sm">
      URLがありません
    </div>

    <div v-else class="divide-y divide-[var(--color-separator)]">
      <div v-for="(item, index) in items" :key="item.id"
           class="flex items-center gap-3 px-4 py-1.5 hover:bg-neutral-50 dark:hover:bg-neutral-800/50 group">
        <span class="text-xs text-neutral-400 w-6 text-right">{{ index + 1 }}</span>
        <div class="flex-1 min-w-0">
          <p class="text-xs font-mono truncate">{{ item.url }}</p>
          <p v-if="item.title" class="text-[10px] text-neutral-500 truncate">{{ item.title }}</p>
        </div>
        <button class="px-2 py-1 rounded text-xs text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 opacity-0 group-hover:opacity-100 transition-opacity"
                @click="handleRemoveItem(item.id)">
          削除
        </button>
      </div>
    </div>
  </div>
</template>
