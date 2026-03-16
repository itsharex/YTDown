import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Download, ViewMode } from '../types'

export const useLibraryStore = defineStore('library', () => {
  const items = ref<Download[]>([])
  const viewMode = ref<ViewMode>('list')
  const searchQuery = ref('')
  const filterFormat = ref<string | null>(null)
  const filterSite = ref<string | null>(null)
  const sortBy = ref<'created_at' | 'title' | 'site'>('created_at')
  const sortOrder = ref<'asc' | 'desc'>('desc')

  const filteredItems = computed(() => {
    let result = items.value.filter(d => d.status === 'completed')
    if (filterFormat.value) {
      result = result.filter(d => d.format === filterFormat.value)
    }
    if (filterSite.value) {
      result = result.filter(d => d.site === filterSite.value)
    }
    return result
  })

  const sites = computed(() =>
    [...new Set(items.value.map(d => d.site).filter(Boolean))] as string[]
  )

  const channels = computed(() => {
    const map = new Map<string, { channel: string; channel_id: string | null; site: string | null }>()
    for (const d of items.value) {
      if (d.channel) {
        const key = `${d.site}:${d.channel_id || d.channel}`
        if (!map.has(key)) {
          map.set(key, { channel: d.channel, channel_id: d.channel_id, site: d.site })
        }
      }
    }
    return [...map.values()]
  })

  return {
    items,
    viewMode,
    searchQuery,
    filterFormat,
    filterSite,
    sortBy,
    sortOrder,
    filteredItems,
    sites,
    channels,
  }
})
