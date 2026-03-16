<script setup lang="ts">
import { useDownloadsStore } from '../../stores/downloads'
import DownloadProgressItem from './DownloadProgress.vue'

const store = useDownloadsStore()

async function handlePause(id: number) { await store.pauseDownload(id) }
async function handleResume(id: number) { await store.resumeDownload(id) }
async function handleCancel(id: number) { await store.cancelDownload(id) }
</script>

<template>
  <div>
    <div v-if="store.activeDownloads.length === 0" class="p-8 text-center text-neutral-400 text-sm">
      ダウンロードはありません
    </div>
    <div v-else class="divide-y divide-[var(--color-separator)]">
      <DownloadProgressItem
        v-for="dl in store.activeDownloads"
        :key="dl.id"
        :download="dl"
        :progress="store.progressMap.get(dl.id)"
        @pause="handlePause"
        @resume="handleResume"
        @cancel="handleCancel"
      />
    </div>
  </div>
</template>
