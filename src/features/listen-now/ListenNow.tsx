import './listenNow.css'
import '../../shared/styles/artwork.css'
import { AlbumShelf } from './components/AlbumShelf'
import { TopPicksShelf } from './components/TopPicksShelf'

export function ListenNow() {
  return (
    <div className="km-listen-now">
      <TopPicksShelf />
      <AlbumShelf shelfKey="recentlyPlayed" />
      <AlbumShelf shelfKey="recentlyAdded" />
    </div>
  )
}
