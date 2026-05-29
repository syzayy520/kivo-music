export type AudioFormat = 'MP3' | 'AAC' | 'ALAC' | 'FLAC' | 'WAV'

export type Track = {
  id: string
  title: string
  artistId: string
  albumId: string
  duration: number
  trackNumber: number
  discNumber?: number
  liked: boolean
  rating?: number
  playCount: number
  lastPlayedAt?: string
  addedAt: string
  format?: AudioFormat
  bitDepth?: number
  sampleRate?: number
  bitrate?: number
  qualityLabel?: string
  lyrics?: string
  syncedLyrics?: SyncedLyricLine[]
  lyricsTranslation?: string
  lyricsPronunciation?: string
  artwork?: string
  localPath?: string
  recommendationWeight?: number
  excludeFromRecommendations?: boolean
}

export type SyncedLyricLine = {
  id: string
  time: number
  text: string
  translation?: string
}

export type Album = {
  id: string
  title: string
  artistId: string
  year?: number
  genre?: string
  trackCount: number
  duration: number
  artwork: string
  isCollected: boolean
  isFavorite: boolean
  rating?: number
  qualityLabel?: string
  format?: string
  releaseType?: 'album' | 'single' | 'ep' | 'compilation'
  discCount?: number
  addedAt: string
  lastPlayedAt?: string
}

export type Artist = {
  id: string
  name: string
  artwork: string
  albumCount: number
  trackCount: number
  genres: string[]
  bio?: string
}

export type Playlist = {
  id: string
  title: string
  description: string
  artwork: string
  trackIds: string[]
  type: 'system' | 'user' | 'smart'
  createdAt: string
  updatedAt: string
}
