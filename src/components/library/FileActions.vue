<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useFileManager } from '../../composables/useFileManager'
import type { Download } from '../../types'

const props = defineProps<{
  item: Download
  x: number
  y: number
}>()

const emit = defineEmits<{ close: [] }>()

const { deleteFile, revealInFinder } = useFileManager()

const showDeleteConfirm = ref(false)

function handleClickOutside(_e: MouseEvent) {
  emit('close')
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

async function handleReveal() {
  if (props.item.file_path) {
    await revealInFinder(props.item.file_path)
  }
  emit('close')
}

async function handleDelete(toTrash: boolean) {
  if (props.item.file_path) {
    await deleteFile(props.item.file_path, toTrash)
  }
  emit('close')
}

function handleMove() {
  // TODO: Implement folder picker dialog via tauri-plugin-dialog
  emit('close')
}

function handleFavorite() {
  // TODO: Toggle favorite via store
  emit('close')
}

function handleAddToPlaylist() {
  // TODO: Show playlist picker
  emit('close')
}
</script>

<template>
  <div class="fixed z-50 bg-white dark:bg-neutral-800 rounded-lg shadow-xl border border-[var(--color-separator)] py-1 min-w-[180px] text-sm"
       :style="{ left: `${x}px`, top: `${y}px` }"
       @click.stop>
    <button class="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700"
            @click="handleReveal">
      Finderで表示
    </button>
    <button class="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700"
            @click="handleMove">
      移動...
    </button>
    <div class="border-t border-[var(--color-separator)] my-1" />
    <button class="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700"
            @click="handleFavorite">
      {{ item.is_favorite ? 'お気に入り解除' : 'お気に入りに追加' }}
    </button>
    <button class="w-full text-left px-3 py-1.5 hover:bg-neutral-100 dark:hover:bg-neutral-700"
            @click="handleAddToPlaylist">
      プレイリストに追加...
    </button>
    <div class="border-t border-[var(--color-separator)] my-1" />
    <template v-if="!showDeleteConfirm">
      <button class="w-full text-left px-3 py-1.5 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-500"
              @click="showDeleteConfirm = true">
        削除
      </button>
    </template>
    <template v-else>
      <button class="w-full text-left px-3 py-1.5 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-500"
              @click="handleDelete(true)">
        ゴミ箱に移動
      </button>
      <button class="w-full text-left px-3 py-1.5 hover:bg-red-50 dark:hover:bg-red-900/20 text-red-600 font-medium"
              @click="handleDelete(false)">
        完全に削除
      </button>
    </template>
  </div>
</template>
