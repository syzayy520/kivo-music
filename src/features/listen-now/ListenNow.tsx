import './listenNow.css'
import '../../shared/styles/artwork.css'
import { AlbumShelf } from './components/AlbumShelf'
import { ListenNowHero } from './components/ListenNowHero'

export function ListenNow() {
  return (
    <>
      <ListenNowHero />
      <AlbumShelf shelfKey="recentlyPlayed" />
      <AlbumShelf shelfKey="recentlyAdded" />
    </>
  )
}
