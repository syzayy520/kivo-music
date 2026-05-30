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
  displayAlbum(heroAlbum, 'top-pick', 'Midnight Archive', 'pink'),
  displayAlbum(albums[1], 'made-for-you', 'UltraBlur Nights', 'violet'),
  displayAlbum(albums[2], 'new-release', 'Static Gardens', 'green'),
]

const recentlyPlayedAlbums: Album[] = [
  displayAlbum(albums[0], 'rp-0', 'Midnight Archive', 'pink'),
  displayAlbum(albums[1], 'rp-1', 'UltraBlur Nights', 'cloud'),
  displayAlbum(albums[2], 'rp-2', 'Static Gardens', 'lake'),
  displayAlbum(albums[0], 'rp-3', 'Neon Prayer', 'moon'),
  displayAlbum(albums[1], 'rp-4', 'Dawn Signals', 'beam'),
  displayAlbum(albums[2], 'rp-5', 'Echo Bloom', 'flowers'),
]

const recentlyAddedAlbums: Album[] = [
  displayAlbum(albums[2], 'ra-0', 'Golden Hour', 'sunset'),
  displayAlbum(albums[1], 'ra-1', 'Velvet Lights', 'hills'),
  displayAlbum(albums[0], 'ra-2', 'River Run', 'ocean'),
  displayAlbum(albums[0], 'ra-3', 'Paper Planes', 'paper'),
  displayAlbum(albums[1], 'ra-4', 'Afterglow', 'afterglow'),
  displayAlbum(albums[2], 'ra-5', 'Frequencies', 'frequency'),
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
