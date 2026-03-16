<script setup lang="ts">
import { computed } from 'vue'
import type { Download, DownloadProgress } from '../../types'

const props = defineProps<{
  download: Download
  progress?: DownloadProgress
}>()

const emit = defineEmits<{
  pause: [id: number]
  resume: [id: number]
  cancel: [id: number]
}>()

const percent = computed(() =>
  props.progress?.percent ?? props.download.progress * 100
)

const statusLabel = computed(() => {
  switch (props.download.status) {
    case 'downloading': return props.progress?.speed_str ?? 'ダウンロード中...'
    case 'paused': return '一時停止中'
    case 'pending': return 'キュー待ち'
    case 'completed': return '完了'
    case 'failed': return props.download.error_message ?? 'エラー'
    default: return props.download.status
  }
})

const isPausable = computed(() => props.download.status === 'downloading')
const isResumable = computed(() => props.download.status === 'paused')
</script>

<template>
  <div class="flex items-center gap-3 px-4 py-2 hover:bg-neutral-50 dark:hover:bg-neutral-800/50">
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium truncate">{{ download.title || download.url }}</p>
      <div class="flex items-center gap-2 mt-1">
        <div class="flex-1 h-1.5 bg-neutral-200 dark:bg-neutral-700 rounded-full overflow-hidden">
          <div class="h-full bg-[var(--color-accent)] rounded-full transition-all"
               :style="{ width: `${percent}%` }" />
        </div>
        <span class="text-xs text-neutral-500 whitespace-nowrap w-12 text-right">
          {{ percent.toFixed(0) }}%
        </span>
      </div>
      <p class="text-xs text-neutral-400 mt-0.5">
        {{ statusLabel }}
        <template v-if="progress?.eta_str"> — 残り {{ progress.eta_str }}</template>
      </p>
    </div>
    <div class="flex gap-1">
      <button v-if="isPausable" @click="emit('pause', download.id)"
              class="p-1 rounded hover:bg-neutral-200 dark:hover:bg-neutral-700 text-xs" title="一時停止">⏸</button>
      <button v-if="isResumable" @click="emit('resume', download.id)"
              class="p-1 rounded hover:bg-neutral-200 dark:hover:bg-neutral-700 text-xs" title="再開">▶</button>
      <button @click="emit('cancel', download.id)"
              class="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-red-500 text-xs" title="キャンセル">&times;</button>
    </div>
  </div>
</template>
