import type { Album } from '../../shared/types/music'
import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'

const heroAlbum = albums[0]

function displayAlbum(album: Album, suffix: string, title: string, artworkKey: string): Album {
  return { ...album, id: `${album.id}-${suffix}`, title, artworkKey }
}

const topPickAlbums: Album[] = [
  displayAlbum(heroAlbum, 'night-library', 'Midnight Archive', 'pink-hill'),
  displayAlbum(albums[1], 'focus-flow', 'Focus Flow', 'purple-cloud'),
  displayAlbum(albums[2], 'quiet-garden', 'Quiet Garden', 'green-hills'),
]

const recentlyPlayedAlbums: Album[] = [
  displayAlbum(albums[0], 'rp-0', 'Midnight Archive', 'pink-hill'),
  displayAlbum(albums[1], 'rp-1', 'UltraBlur Nights', 'purple-cloud'),
  displayAlbum(albums[2], 'rp-2', 'Static Gardens', 'summer-lake'),
  displayAlbum(albums[0], 'rp-3', 'Neon Prayer', 'moon-pink'),
  displayAlbum(albums[1], 'rp-4', 'Dawn Signals', 'beam-sky'),
  displayAlbum(albums[2], 'rp-5', 'Echo Bloom', 'spring-bloom'),
]

const recentlyAddedAlbums: Album[] = [
  displayAlbum(albums[2], 'ra-0', 'Golden Hour', 'autumn-sunset'),
  displayAlbum(albums[1], 'ra-1', 'Velvet Lights', 'winter-mist'),
  displayAlbum(albums[0], 'ra-2', 'River Run', 'ocean-wave'),
  displayAlbum(albums[0], 'ra-3', 'Paper Planes', 'paper-plane'),
  displayAlbum(albums[1], 'ra-4', 'Afterglow', 'moon-pink'),
  displayAlbum(albums[2], 'ra-5', 'Frequencies', 'frequency-blue'),
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
