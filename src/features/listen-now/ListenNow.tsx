import './styles/listenNowStyles.css'
import '../../shared/styles/artwork.css'
import { AlbumShelf } from './components/AlbumShelf'
import { TopPicksShelf } from './components/TopPicksShelf'
import { listenNowStyle } from './listenNowStyle'

export function ListenNow() {
  return (
    <div className="km-listen-now" style={listenNowStyle}>
      <TopPicksShelf />
      <AlbumShelf shelfKey="recentlyPlayed" />
      <AlbumShelf shelfKey="recentlyAdded" />
    </div>
  )
}
