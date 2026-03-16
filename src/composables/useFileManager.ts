import { invoke } from '@tauri-apps/api/core'

export function useFileManager() {
  async function moveFile(source: string, destination: string) {
    return invoke('move_file', { source, destination })
  }

  async function deleteFile(path: string, toTrash: boolean = true) {
    return invoke('delete_file', { path, toTrash })
  }

  async function revealInFinder(path: string) {
    return invoke('reveal_in_finder', { path })
  }

  return { moveFile, deleteFile, revealInFinder }
}
