import { Sidebar } from '../features/sidebar/Sidebar'
import { TopBar } from '../features/top-bar/TopBar'
import { ListenNowPage } from '../features/listen-now/ListenNowPage'
import { AlbumDetailPage } from '../features/album-detail/AlbumDetailPage'
import { NowPlayingSection } from '../features/now-playing/NowPlayingSection'
import { PlayerBar } from '../features/player/PlayerBar'

export function App() {
  return (
    <div className="kivo-app">
      <div className="kivo-shell">
        <Sidebar />
        <div className="kivo-main">
          <TopBar />
          <ListenNowPage />
          <AlbumDetailPage />
          <NowPlayingSection />
        </div>
      </div>
      <PlayerBar />
    </div>
  )
}
