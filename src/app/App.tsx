import { AlbumDetail } from '../features/album-detail/AlbumDetail'
import { LibraryOverview } from '../features/library/LibraryOverview'
import { ListenNow } from '../features/listen-now/ListenNow'
import { NowPlaying } from '../features/now-playing/NowPlaying'
import { PlayerBar } from '../features/player/PlayerBar'
import { SettingsPreview } from '../features/settings/SettingsPreview'
import { Sidebar } from '../features/sidebar/Sidebar'
import { TopBar } from '../features/top-bar/TopBar'

export function App() {
  return (
    <main className="km-app" aria-label="Kivo Music">
      <Sidebar />
      <section className="km-stage">
        <TopBar />
        <ListenNow />
        <AlbumDetail />
        <NowPlaying />
        <LibraryOverview />
        <SettingsPreview />
      </section>
      <PlayerBar />
    </main>
  )
}
