<script setup lang="ts">
import { ref } from 'vue'
import { usePlaylistsStore } from '../../stores/playlists'

const emit = defineEmits<{
  select: [id: number]
  'select-url-list': [id: number]
}>()

const store = usePlaylistsStore()

const newPlaylistName = ref('')
const showNewInput = ref(false)
const activeTab = ref<'playlists' | 'url-lists'>('playlists')

async function createPlaylist() {
  if (!newPlaylistName.value.trim()) return
  // TODO: invoke create_playlist command
  showNewInput.value = false
  newPlaylistName.value = ''
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') createPlaylist()
  if (e.key === 'Escape') { showNewInput.value = false; newPlaylistName.value = '' }
}
</script>

<template>
  <div class="space-y-2">
    <!-- Tab toggle -->
    <div class="flex gap-1 px-1">
      <button class="flex-1 px-2 py-1 text-xs rounded"
              :class="activeTab === 'playlists' ? 'bg-white dark:bg-neutral-700 shadow-sm' : 'text-neutral-500'"
              @click="activeTab = 'playlists'">
        プレイリスト
      </button>
      <button class="flex-1 px-2 py-1 text-xs rounded"
              :class="activeTab === 'url-lists' ? 'bg-white dark:bg-neutral-700 shadow-sm' : 'text-neutral-500'"
              @click="activeTab = 'url-lists'">
        URLリスト
      </button>
    </div>

    <!-- Playlists -->
    <template v-if="activeTab === 'playlists'">
      <ul class="space-y-0.5">
        <li v-for="pl in store.playlists" :key="pl.id">
          <button class="w-full text-left px-2 py-1 rounded-md hover:bg-black/5 dark:hover:bg-white/5 text-sm truncate"
                  @click="emit('select', pl.id)">
            {{ pl.name }}
          </button>
        </li>
      </ul>

      <!-- New playlist input -->
      <div v-if="showNewInput" class="px-1">
        <input v-model="newPlaylistName"
               class="w-full h-7 px-2 text-xs rounded-md bg-neutral-100 dark:bg-neutral-700 outline-none focus:ring-1 focus:ring-[var(--color-accent)]"
               placeholder="プレイリスト名"
               @keydown="handleKeydown"
               autofocus />
      </div>
      <button v-else
              class="w-full text-left px-2 py-1 text-xs text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
              @click="showNewInput = true">
        + 新規プレイリスト
      </button>
    </template>

    <!-- URL Lists -->
    <template v-else>
      <ul class="space-y-0.5">
        <li v-for="ul in store.urlLists" :key="ul.id">
          <button class="w-full text-left px-2 py-1 rounded-md hover:bg-black/5 dark:hover:bg-white/5 text-sm truncate"
                  @click="emit('select-url-list', ul.id)">
            {{ ul.name }}
          </button>
        </li>
      </ul>
      <button class="w-full text-left px-2 py-1 text-xs text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300">
        + 新規URLリスト
      </button>
    </template>
  </div>
</template>
