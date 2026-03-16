import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Playlist, PlaylistItem, UrlList, UrlListItem } from '../types'

export const usePlaylistsStore = defineStore('playlists', () => {
  const playlists = ref<Playlist[]>([])
  const currentPlaylistItems = ref<PlaylistItem[]>([])
  const urlLists = ref<UrlList[]>([])
  const currentUrlListItems = ref<UrlListItem[]>([])

  return {
    playlists,
    currentPlaylistItems,
    urlLists,
    currentUrlListItems,
  }
})
