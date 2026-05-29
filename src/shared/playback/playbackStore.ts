import type { PlaybackState } from './playbackTypes'

export const initialPlaybackState: PlaybackState = {
  status: 'Idle',
  current_track: null,
  timeline: {
    position_ms: 0,
    duration_ms: null,
    progress_event_interval_ms: 500,
  },
  volume: {
    level: 1,
    muted: false,
  },
  metadata: null,
  error: null,
}
