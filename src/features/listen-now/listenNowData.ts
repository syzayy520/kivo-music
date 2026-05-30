import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'

const heroAlbum = albums[0]
const homeShelfAlbums = [...albums, ...albums]
const recentShelfAlbums = [...albums].reverse().concat(albums)

export const listenNowData = {
  heroAlbum,
  heroArtist: artists.find((artist) => artist.id === heroAlbum.artistId),
  shelves: {
    recentlyPlayed: homeShelfAlbums,
    recentlyAdded: recentShelfAlbums,
  },
  artists,
}
