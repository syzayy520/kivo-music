import { artists } from '../../data/mock/artists'
import { tracks } from '../../data/mock/tracks'
import { playerState } from './playerState'

const track = tracks.find((item) => item.id === playerState.currentTrackId) ?? tracks[0]
const artist = artists.find((item) => item.id === track.artistId)

export const playerData = {
  track,
  artist,
  state: playerState,
}
