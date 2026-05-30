import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'

const heroAlbum = albums[0]

export const listenNowData = {
  heroAlbum,
  heroArtist: artists.find((artist) => artist.id === heroAlbum.artistId),
  shelves: {
    recentlyPlayed: albums,
    recentlyAdded: [...albums].reverse(),
  },
  artists,
}
