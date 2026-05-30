import './listenNow.css'
import '../../shared/styles/artwork.css'
import { AlbumShelf } from './components/AlbumShelf'
import { StartListeningPanel } from './components/StartListeningPanel'
import { TopPicksShelf } from './components/TopPicksShelf'

export function ListenNow() {
  return (
    <div className="km-listen-now">
      <StartListeningPanel />
      <TopPicksShelf />
      <AlbumShelf shelfKey="recentlyPlayed" />
      <AlbumShelf shelfKey="recentlyAdded" />
    </div>
  )
}
