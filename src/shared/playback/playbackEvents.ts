import type { PlaybackState } from './playbackTypes'

export type PlaybackEvent =
  | { type: 'StateChanged'; state: PlaybackState }
  | { type: 'Progress'; position_ms: number; duration_ms: number | null }
  | { type: 'TrackChanged'; state: PlaybackState }
  | { type: 'Error'; error: string }

export type PlaybackEventHandler = (event: PlaybackEvent) => void

export function subscribePlaybackEvents(_handler: PlaybackEventHandler) {
  return () => undefined
}
