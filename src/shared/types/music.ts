export type AudioFormat = 'MP3' | 'AAC' | 'ALAC' | 'FLAC' | 'WAV'

export type SyncedLyricLine = {
  id: string
  time: number
  text: string
  translation?: string
}

export type Track = {
  id: string
  title: string
  artistId: string
  albumId: string
  duration: number
  trackNumber: number
  liked: boolean
  rating?: number
  playCount: number
  addedAt: string
  format?: AudioFormat
  bitDepth?: number
  sampleRate?: number
  qualityLabel?: string
  syncedLyrics?: SyncedLyricLine[]
}

export type Album = {
  id: string
  title: string
  artistId: string
  year?: number
  genre?: string
  trackCount: number
  duration: number
  artworkKey: string
  isCollected: boolean
  isFavorite: boolean
  rating?: number
  qualityLabel?: string
  format?: AudioFormat
  addedAt: string
}

export type Artist = {
  id: string
  name: string
  artworkKey: string
  albumCount: number
  trackCount: number
  genres: string[]
}

export type Playlist = {
  id: string
  title: string
  description: string
  artworkKey: string
  trackIds: string[]
  type: 'system' | 'user' | 'smart'
}
