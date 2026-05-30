import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'
import { tracks } from '../../data/mock/tracks'
import { resolveArtworkStyle } from '../../shared/artwork/resolveArtworkClass'
import { playerState } from './playerState'

const currentTrack = tracks.find((track) => track.id === playerState.currentTrackId) ?? tracks[0]
const currentAlbum = albums.find((album) => album.id === currentTrack.albumId)
const currentAlbumIndex = Math.max(
  0,
  albums.findIndex((album) => album.id === currentTrack.albumId),
)
const currentArtworkSource = currentAlbum ?? {}
const qualityShortLabel = playerState.qualityLabel.split(' · ')[0] ?? playerState.qualityLabel

export const playerData = {
  track: currentTrack,
  artist: artists.find((artist) => artist.id === currentTrack.artistId),
  album: currentAlbum,
  artworkStyle: resolveArtworkStyle(currentArtworkSource, currentAlbumIndex),
  qualityShortLabel,
  state: playerState,
}
