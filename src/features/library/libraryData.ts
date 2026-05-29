import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'
import { playlists } from '../../data/mock/playlists'
import { tracks } from '../../data/mock/tracks'

export const libraryData = {
  stats: [
    { label: 'Albums', value: albums.length.toString() },
    { label: 'Artists', value: artists.length.toString() },
    { label: 'Songs', value: tracks.length.toString() },
    { label: 'Playlists', value: playlists.length.toString() },
  ],
  recentlyAddedAlbums: [...albums].sort((a, b) => b.addedAt.localeCompare(a.addedAt)),
}
