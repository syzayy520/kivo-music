import { tracks } from '../../data/mock/tracks'
import { playerState } from '../player/playerState'

export const queueData = {
  tracks: playerState.queue
    .map((trackId) => tracks.find((track) => track.id === trackId))
    .filter((track) => track !== undefined),
}
