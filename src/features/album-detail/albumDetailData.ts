import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'
import { tracks } from '../../data/mock/tracks'

const album = albums[0]

export const albumDetailData = {
  album,
  artist: artists.find((artist) => artist.id === album.artistId),
  tracks: tracks.filter((track) => track.albumId === album.id),
}
