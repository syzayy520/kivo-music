export type PlaybackStatus = 'Idle' | 'Loading' | 'Playing' | 'Paused' | 'Stopped' | 'Failed'

export type PlaybackTimeline = {
  position_ms: number
  duration_ms: number | null
  progress_event_interval_ms: number
}

export type PlaybackVolume = {
  level: number
  muted: boolean
}

export type PlaybackTrack = {
  id: string
  title: string
  artist: string
  source_path: string
}

export type AudioMetadata = {
  codec: string | null
  container: string | null
  sample_rate_hz: number | null
  bit_depth: number | null
  channels: number | null
  bitrate_kbps: number | null
  is_lossless: boolean | null
}

export type PlaybackState = {
  status: PlaybackStatus
  current_track: PlaybackTrack | null
  timeline: PlaybackTimeline
  volume: PlaybackVolume
  metadata: AudioMetadata | null
  error: string | null
}
