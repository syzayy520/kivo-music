import { artists } from '../../data/mock/artists'
import { tracks } from '../../data/mock/tracks'
import { playerState } from './playerState'

const currentTrack = tracks.find((track) => track.id === playerState.currentTrackId) ?? tracks[0]

export const playerData = {
  track: currentTrack,
  artist: artists.find((artist) => artist.id === currentTrack.artistId),
  state: playerState,
}
