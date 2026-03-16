import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Download, DownloadProgress, DownloadOptions, VideoInfo } from '../types'

export const useDownloadsStore = defineStore('downloads', () => {
  const queue = ref<Download[]>([])
  const progressMap = ref<Map<number, DownloadProgress>>(new Map())

  const activeDownloads = computed(() =>
    queue.value.filter(d => ['downloading', 'paused', 'pending'].includes(d.status))
  )
  const completedDownloads = computed(() =>
    queue.value.filter(d => d.status === 'completed')
  )

  async function fetchFormats(url: string): Promise<VideoInfo> {
    return invoke<VideoInfo>('fetch_formats', { url })
  }

  async function startDownload(url: string, options: DownloadOptions): Promise<number> {
    const id = await invoke<number>('start_download', { url, options })
    return id
  }

  async function cancelDownload(downloadId: number) {
    await invoke('cancel_download', { downloadId })
  }

  async function pauseDownload(downloadId: number) {
    await invoke('pause_download', { downloadId })
  }

  async function resumeDownload(downloadId: number) {
    await invoke('resume_download', { downloadId })
  }

  let unlistenFn: (() => void) | null = null

  async function setupProgressListener() {
    if (unlistenFn) unlistenFn()
    unlistenFn = await listen<DownloadProgress>('download-progress', (event) => {
      progressMap.value.set(event.payload.download_id, event.payload)
    })
  }

  function cleanup() {
    if (unlistenFn) { unlistenFn(); unlistenFn = null }
  }

  return {
    queue,
    progressMap,
    activeDownloads,
    completedDownloads,
    fetchFormats,
    startDownload,
    cancelDownload,
    pauseDownload,
    resumeDownload,
    setupProgressListener,
    cleanup,
  }
})
