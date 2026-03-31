import { Dock } from "../components/Dock";
import { LibraryPanels } from "../components/LibraryPanels";
import { PlayerHero } from "../components/PlayerHero";
import { QueuePanel } from "../components/QueuePanel";
import { SearchToolbar } from "../components/SearchToolbar";
import { Sidebar } from "../components/Sidebar";
import { WorkspaceScrollbar } from "../components/WorkspaceScrollbar";
import { useFileDrop } from "../hooks/useFileDrop";
import { useLibrary } from "../hooks/useLibrary";
import { usePlayer } from "../hooks/usePlayer";
import { useWorkspaceScroll } from "../hooks/useWorkspaceScroll";

export function AppShell() {
  const library = useLibrary();
  const player = usePlayer();
  const scroll = useWorkspaceScroll();
  const countsLabel = `${library.counts.tracks.toLocaleString("zh-CN")} 首音乐 · ${library.counts.roots} 个资料夹`;
  useFileDrop(player.playDroppedFile, player.resolveDroppedPath);

  return (
    <main className="app-shell">
      <div className="window">
        <div className="window-content">
          <Sidebar countsLabel={library.isScanning ? "正在扫描资料库…" : countsLabel} onLibraryAction={(label) => label === "本地导入" && void library.scanLibrary()} />
          <section className="main-pane">
            <SearchToolbar loading={library.searching} onQueryChange={library.setQuery} query={library.query} results={library.results} />
            <div className="workspace-shell">
              <div className="workspace" ref={scroll.workspaceRef}>
                <PlayerHero onSeek={player.setProgress} onTogglePlay={player.togglePlay} playback={player.playback} queueIndex={player.currentIndex} track={player.currentTrack} />
                <section className="split">
                  <LibraryPanels />
                  <QueuePanel currentId={player.currentId} onSelect={player.selectTrack} queue={player.queue} />
                </section>
              </div>
              <WorkspaceScrollbar thumbHeight={scroll.metrics.thumbHeight} thumbTop={scroll.metrics.thumbTop} visible={scroll.metrics.visible} />
            </div>
          </section>
          <Dock onSeek={player.setProgress} onSeekBack={player.seekBack} onSeekForward={player.seekForward} onTogglePlay={player.togglePlay} onVolume={player.setVolume} playback={player.playback} track={player.currentTrack} />
        </div>
      </div>
    </main>
  );
}
