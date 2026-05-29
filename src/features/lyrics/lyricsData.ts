import { tracks } from '../../data/mock/tracks'
import { playerState } from '../player/playerState'

const currentTrack = tracks.find((track) => track.id === playerState.currentTrackId) ?? tracks[0]

export const lyricsData = {
  track: currentTrack,
  lines: currentTrack.syncedLyrics ?? [],
  activeLineId: currentTrack.syncedLyrics?.[1]?.id,
}
