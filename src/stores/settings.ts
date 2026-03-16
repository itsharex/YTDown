import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AppSettings } from '../types'

const DEFAULTS: AppSettings = {
  download_dir: '~/Downloads/YTDown/',
  filename_template: '%(title)s.%(ext)s',
  concurrent_downloads: 3,
  default_video_format: 'mp4',
  default_video_quality: 'best',
  default_audio_format: 'mp3',
  embed_thumbnail: true,
  embed_metadata: true,
  write_subs: false,
  embed_subs: false,
  embed_chapters: true,
  sponsorblock: false,
  cookie_browser: 'none',
  cookie_file: '',
  ytdlp_path: 'auto',
  theme: 'system',
  auto_classify: false,
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({ ...DEFAULTS })
  const loaded = ref(false)

  // Settings are loaded/saved via tauri-plugin-sql
  // The SQL queries will be called from composables

  function updateSetting<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
    settings.value[key] = value
  }

  return { settings, loaded, updateSetting }
})
