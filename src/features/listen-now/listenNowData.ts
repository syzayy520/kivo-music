import type { Album } from '../../shared/types/music'
import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'

const heroAlbum = albums[0]

function displayAlbum(album: Album, suffix: string, title: string, artworkKey: string): Album {
  return {
    ...album,
    id: `${album.id}-${suffix}`,
    title,
    artworkKey,
  }
}

const topPickAlbums: Album[] = [
  heroAlbum,
  displayAlbum(albums[1], 'made-for-you', 'UltraBlur Nights', 'violet'),
  displayAlbum(albums[2], 'new-release', 'Static Gardens', 'green'),
]

const recentlyPlayedAlbums: Album[] = [
  albums[0],
  albums[1],
  albums[2],
  displayAlbum(albums[0], 'neon-prayer', 'Neon Prayer', 'pink'),
  displayAlbum(albums[1], 'dawn-signals', 'Dawn Signals', 'violet'),
  displayAlbum(albums[2], 'echo-bloom', 'Echo Bloom', 'green'),
]

const recentlyAddedAlbums: Album[] = [
  albums[2],
  albums[1],
  albums[0],
  displayAlbum(albums[0], 'afterimage', 'Afterimage Radio', 'pink'),
  displayAlbum(albums[1], 'velvet', 'Velvet Signal', 'violet'),
  displayAlbum(albums[2], 'city-lights', 'City Lights', 'green'),
]

export const listenNowData = {
  heroAlbum,
  heroArtist: artists.find((artist) => artist.id === heroAlbum.artistId),
  topPicks: topPickAlbums,
  shelves: {
    recentlyPlayed: recentlyPlayedAlbums,
    recentlyAdded: recentlyAddedAlbums,
  },
  artists,
}
