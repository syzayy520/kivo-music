export type PlaybackCommand = 'play' | 'pause' | 'seek' | 'next' | 'previous' | 'setQueue'

export type PlaybackCoreState = {
  currentTrackId?: string
  isPlaying: boolean
  progress: number
  duration: number
  volume: number
  queue: string[]
  outputDevice?: string
  qualityLabel?: string
  diagnostics?: string[]
}

export type PlaybackCoreBoundary = {
  command: PlaybackCommand
  state: PlaybackCoreState
}
