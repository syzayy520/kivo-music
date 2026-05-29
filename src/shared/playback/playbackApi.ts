import { invoke } from '@tauri-apps/api/core'
import type { PlaybackState } from './playbackTypes'

export function getPlaybackState() {
  return invoke<PlaybackState>('playback_get_state')
}
