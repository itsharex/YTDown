<script setup lang="ts">
import { computed } from 'vue'
import { usePlaylistsStore } from '../../stores/playlists'
import type { Playlist } from '../../types'

defineProps<{
  playlist: Playlist | null
}>()

const emit = defineEmits<{
  'download-url': [url: string]
}>()

const store = usePlaylistsStore()

const items = computed(() => store.currentPlaylistItems)

const hasUrlOnlyItems = computed(() =>
  items.value.some(item => item.download_id === null && item.url)
)

function handleDownloadMissing() {
  for (const item of items.value) {
    if (item.download_id === null && item.url) {
      emit('download-url', item.url)
    }
  }
}

async function handleRemoveItem(_itemId: number) {
  // TODO: invoke remove_from_playlist command
}
</script>

<template>
  <div v-if="!playlist" class="p-8 text-center text-neutral-400 text-sm">
    プレイリストを選択してください
  </div>

  <div v-else>
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-[var(--color-separator)]">
      <div>
        <h2 class="text-lg font-semibold">{{ playlist.name }}</h2>
        <p v-if="playlist.description" class="text-sm text-neutral-500">{{ playlist.description }}</p>
        <p class="text-xs text-neutral-400 mt-0.5">{{ items.length }} アイテム</p>
      </div>
      <button v-if="hasUrlOnlyItems"
              class="px-3 py-1.5 rounded-md text-xs bg-[var(--color-accent)] text-white"
              @click="handleDownloadMissing">
        未ダウンロードを取得
      </button>
    </div>

    <!-- Items list -->
    <div v-if="items.length === 0" class="p-8 text-center text-neutral-400 text-sm">
      アイテムがありません
    </div>

    <div v-else class="divide-y divide-[var(--color-separator)]">
      <div v-for="(item, index) in items" :key="item.id"
           class="flex items-center gap-3 px-4 py-2 hover:bg-neutral-50 dark:hover:bg-neutral-800/50 group">
        <!-- Order number -->
        <span class="text-xs text-neutral-400 w-6 text-right">{{ index + 1 }}</span>

        <!-- Info -->
        <div class="flex-1 min-w-0">
          <p class="text-sm truncate">
            {{ item.url ?? `ダウンロード #${item.download_id}` }}
          </p>
          <p class="text-xs text-neutral-500">
            <span v-if="item.download_id" class="text-green-600 dark:text-green-400">ダウンロード済み</span>
            <span v-else class="text-neutral-400">URL のみ</span>
          </p>
        </div>

        <!-- Actions -->
        <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          <button v-if="!item.download_id && item.url"
                  class="px-2 py-1 rounded text-xs bg-[var(--color-accent)] text-white"
                  @click="emit('download-url', item.url)">
            DL
          </button>
          <button class="px-2 py-1 rounded text-xs text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20"
                  @click="handleRemoveItem(item.id)">
            削除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
