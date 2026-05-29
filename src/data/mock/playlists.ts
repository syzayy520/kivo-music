import type { Playlist } from '../../shared/types/music'

export const playlists: Playlist[] = [
  {
    id: 'playlist-liked-songs',
    title: 'Liked Songs',
    description: 'Songs marked as loved, separated from album collection state.',
    artworkKey: 'pink',
    trackIds: ['track-neon-prayer', 'track-afterimage-radio'],
    type: 'system',
  },
  {
    id: 'playlist-midnight-drive',
    title: 'Midnight Drive',
    description: 'A calm local-library mix for late rooms and long roads.',
    artworkKey: 'violet',
    trackIds: ['track-neon-prayer', 'track-velvet-signal', 'track-afterimage-radio'],
    type: 'smart',
  },
]
