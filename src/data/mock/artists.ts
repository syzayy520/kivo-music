import type { Artist } from '../../shared/types/music'

export const artists: Artist[] = [
  {
    id: 'a-luna',
    name: 'Luna Vale',
    artwork: 'linear-gradient(135deg, #ff4fa3, #8e65ff)',
    albumCount: 4,
    trackCount: 48,
    genres: ['Dream Pop', 'Alternative'],
    bio: 'A nocturnal voice built for late-night rooms and cinematic synths.',
  },
  {
    id: 'a-orbit',
    name: 'Orbit Room',
    artwork: 'linear-gradient(135deg, #ff3b5c, #ffd166)',
    albumCount: 3,
    trackCount: 34,
    genres: ['Electronic', 'Ambient'],
  },
  {
    id: 'a-nova',
    name: 'Nova District',
    artwork: 'linear-gradient(135deg, #4cc9f0, #8e65ff)',
    albumCount: 5,
    trackCount: 61,
    genres: ['Indie', 'Rock'],
  },
]
