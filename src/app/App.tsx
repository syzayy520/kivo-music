import { HomeExperienceHost } from '../features/home/HomeExperienceHost'
import { PlayerBar } from '../features/player/PlayerBar'
import { Sidebar } from '../features/sidebar/Sidebar'
import { TopBar } from '../features/top-bar/TopBar'
import { WindowChrome } from '../features/window-chrome/WindowChrome'
import { appShellStyle } from './appShellStyle'
import './appShell.css'

export function App() {
  return (
    <main className="km-app" style={appShellStyle}>
      <WindowChrome />
      <Sidebar />
      <section className="km-stage">
        <TopBar />
        <HomeExperienceHost />
      </section>
      <PlayerBar />
    </main>
  )
}
