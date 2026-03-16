import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { YtdlpInfo } from '../types'

export function useYtdlp() {
  const info = ref<YtdlpInfo | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadInfo() {
    loading.value = true
    error.value = null
    try {
      info.value = await invoke<YtdlpInfo>('get_ytdlp_info')
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function checkUpdate(): Promise<boolean> {
    return invoke<boolean>('check_ytdlp_update')
  }

  return { info, loading, error, loadInfo, checkUpdate }
}
