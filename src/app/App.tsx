import { AlbumDetail } from '../features/album-detail/AlbumDetail'
import { ListenNow } from '../features/listen-now/ListenNow'
import { NowPlaying } from '../features/now-playing/NowPlaying'
import { PlayerBar } from '../features/player/PlayerBar'
import { Sidebar } from '../features/sidebar/Sidebar'
import { TopBar } from '../features/top-bar/TopBar'
import './appShell.css'

export function App() {
  return (
    <main className="km-app">
      <Sidebar />
      <section className="km-stage">
        <TopBar />
        <ListenNow />
        <AlbumDetail />
        <NowPlaying />
      </section>
      <PlayerBar />
    </main>
  )
}
