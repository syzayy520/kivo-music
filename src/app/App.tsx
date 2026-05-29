import { ListenNow } from '../features/listen-now/ListenNow'
import { PlayerBar } from '../features/player/PlayerBar'
import { Sidebar } from '../features/sidebar/Sidebar'
import { TopBar } from '../features/top-bar/TopBar'
import { WindowChrome } from '../features/window-chrome/WindowChrome'
import './appShell.css'

export function App() {
  return (
    <main className="km-app">
      <WindowChrome />
      <Sidebar />
      <section className="km-stage">
        <TopBar />
        <ListenNow />
      </section>
      <PlayerBar />
    </main>
  )
}
