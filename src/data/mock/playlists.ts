import type { Playlist } from '../../shared/types/music'

export const playlists: Playlist[] = [
  {
    id: 'playlist-liked',
    title: 'Liked Songs',
    description: 'Songs you marked as loved, separated from album collection state.',
    artworkClass: 'artwork-pink',
    trackIds: ['track-neon-prayer', 'track-afterimage'],
    type: 'system',
    createdAt: '2026-05-01',
    updatedAt: '2026-05-29',
  },
  {
    id: 'playlist-midnight-drive',
    title: 'Midnight Drive',
    description: 'A calm local-library mix for late rooms and long roads.',
    artworkClass: 'artwork-violet',
    trackIds: ['track-neon-prayer', 'track-velvet-signal', 'track-afterimage'],
    type: 'smart',
    createdAt: '2026-05-17',
    updatedAt: '2026-05-28',
  },
]
