import type { Artist } from '../../shared/types/music'

export const artists: Artist[] = [
  {
    id: 'a-luna',
    name: 'Luna Vale',
    artworkClass: 'artwork-pink',
    albumCount: 4,
    trackCount: 48,
    genres: ['Dream Pop', 'Alternative'],
    bio: 'A nocturnal voice built for late-night rooms and cinematic synths.',
  },
  {
    id: 'a-orbit',
    name: 'Orbit Room',
    artworkClass: 'artwork-violet',
    albumCount: 3,
    trackCount: 34,
    genres: ['Electronic', 'Ambient'],
  },
  {
    id: 'a-nova',
    name: 'Nova District',
    artworkClass: 'artwork-green',
    albumCount: 5,
    trackCount: 61,
    genres: ['Indie', 'Rock'],
  },
]
