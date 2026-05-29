import { albums } from '../../data/mock/albums'
import { artists } from '../../data/mock/artists'
import { playlists } from '../../data/mock/playlists'
import { tracks } from '../../data/mock/tracks'

export function LibraryOverview() {
  return (
    <section className="km-library" id="library">
      <p>Library Foundation</p>
      <h2>Local collection surfaces are ready for real scanning.</h2>
      <div>
        <article><strong>{albums.length}</strong><span>Albums</span></article>
        <article><strong>{artists.length}</strong><span>Artists</span></article>
        <article><strong>{tracks.length}</strong><span>Tracks</span></article>
        <article><strong>{playlists.length}</strong><span>Playlists</span></article>
      </div>
    </section>
  )
}
