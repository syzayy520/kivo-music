import { HomeExperienceHost } from '../features/home/HomeExperienceHost'
import { ACTIVE_HOME_EXPERIENCE_SHELL_MODE } from '../features/home/homeExperienceRegistry'
import { PlayerBar } from '../features/player/PlayerBar'
import { Sidebar } from '../features/sidebar/Sidebar'
import { TopBar } from '../features/top-bar/TopBar'
import { WindowChrome } from '../features/window-chrome/WindowChrome'
import { appShellStyle } from './appShellStyle'
import './appShell.css'

export function App() {
  return (
    <main
      className="km-app"
      data-shell-mode={ACTIVE_HOME_EXPERIENCE_SHELL_MODE}
      style={appShellStyle}
    >
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
