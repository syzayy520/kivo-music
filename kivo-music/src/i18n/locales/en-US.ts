// src/i18n/locales/en-US.ts
/**
 * English messages.
 */
export const enUSMessages: Record<string, string> = {
  // Mini player
  "miniPlayer.button.exit": "Back to full mode",
  "miniPlayer.cover.placeholder": "No artwork",
  "miniPlayer.header.subtitle.hasTrack": "Current track",
  "miniPlayer.header.subtitle.noTrack": "No track",
  "miniPlayer.header.title": "Kivo Music · Mini",
  "miniPlayer.volume.label": "Volume",

  // Navigation
  "nav.library.label": "Library",
  "nav.library.description":
    "Browse your local music by songs, albums or artists.",
  "nav.playlist.label": "Playlists & Smart lists",
  "nav.playlist.description":
    "Manage playlists and smart lists powered by your play history.",
  "nav.nowPlaying.label": "Now Playing",
  "nav.nowPlaying.description":
    "Focused playback view with current track and queue.",
  "nav.settings.label": "Settings",
  "nav.settings.description":
    "Cover cache, logs and experimental features.",

  // Library · Header
  "library.header.title": "Local music library",
  "library.header.subtitle.line1": "Organize your local music library.",
  "library.header.subtitle.line2":
    "Switch views by songs, albums or artists, with search, sort and play queue support.",
  "library.header.viewMode.tracks": "By songs",
  "library.header.viewMode.albums": "By albums",
  "library.header.viewMode.artists": "By artists",
  "library.header.searchPlaceholder":
    "Search songs / artists / albums / path…",
  "library.header.sort.default": "Default order",
  "library.header.sort.title": "Title",
  "library.header.sort.artist": "Artist",
  "library.header.sort.album": "Album",
  "library.header.sort.recent": "Recently played",
  "library.header.actions.playAll": "Play all",
  "library.header.actions.shufflePlay": "Shuffle play",
  "library.header.actions.importLocalMusic": "+ Import local music",
  "library.header.actions.clearLibrary": "Clear library",
  "library.header.extra.totalTracks": "Total {count} tracks",

  // Library · Sort bar
  "library.sortBar.option.none": "Default order",
  "library.sortBar.option.title": "Title",
  "library.sortBar.option.artist": "Artist",
  "library.sortBar.hint.doubleClickToPlay":
    "Double-click any row to play and use the current filtered result as the play queue.",

  // Playlists · Header
  "playlists.header.title": "Playlists & Smart lists",
  "playlists.header.description":
    "Currently {count} tracks in total, with views by queue, recently played, most played, etc.",
  "playlists.header.smartList.hint":
    "Smart lists are auto-updated based on your play history.",
  "playlists.header.nowPlaying.badge": "Now playing",
  "playlists.header.nowPlaying.title.fallback": "No track is currently playing",
  "playlists.header.nowPlaying.artist.fallback": "Unknown artist",
  "playlists.header.clearQueueButton": "Clear current queue",

  // Settings · General
  "settings.general.title": "General",
  "settings.general.theme.cardTitle": "Theme",
  "settings.general.theme.cardDescription":
    "Switch between light, dark and system theme. Only affects appearance, not playback behavior.",
  "settings.general.theme.system": "Follow system",
  "settings.general.theme.light": "Light",
  "settings.general.theme.dark": "Dark",
  "settings.general.language.cardTitle": "Interface language",
  "settings.general.language.cardDescription":
    "Follow system language by default, or choose manually to test multi-language UI.",
  "settings.general.language.system": "Follow system",
  "settings.general.language.zhCN": "Simplified Chinese",
  "settings.general.language.enUS": "English",

  // Now Playing · Header
  "nowPlaying.header.title": "Now Playing",
  "nowPlaying.header.subtitle.line1":
    "Focused playback view with large artwork, track details and play queue.",
  "nowPlaying.header.subtitle.line2":
    "The current queue comes from your actions (library, playlists, smart lists, etc.).",
  "nowPlaying.header.queueEmpty":
    "The play queue is empty. Start playback from Library or Playlists.",
  "nowPlaying.header.note":
    "This page only renders the view. All playback is driven by the bottom player bar.",

  // Now Playing · Info panel
  "nowPlaying.info.fallbackTitle": "No track selected",
  "nowPlaying.info.fallbackArtist": "Unknown artist",
  "nowPlaying.info.description.line1":
    "This page, the bottom player bar and Mini mode share the same playback state.",
  "nowPlaying.info.description.line2":
    "Actions in Library / Playlists (play / skip) are reflected here in real time.",
  "nowPlaying.info.statusLabel": "Status:",
  "nowPlaying.info.status.playing": "Playing",
  "nowPlaying.info.status.paused": "Paused",
  "nowPlaying.info.status.noTrack": "No track selected",

  // Now Playing · Visualizer panel (example)
  "nowPlaying.visualizer.title": "Audio visualizer",
  "nowPlaying.visualizer.status.enabled": "Enabled",
  "nowPlaying.visualizer.status.disabled": "Disabled",

  // Player bar
  "player.bar.fallbackTitle": "No track selected",
  "player.bar.fallbackArtist": "Unknown artist",
  "player.bar.tooltip.previous": "Previous",
  "player.bar.tooltip.play": "Play",
  "player.bar.tooltip.pause": "Pause",
  "player.bar.tooltip.next": "Next",
  "player.bar.volume.label": "Volume",

  // Sidebar & footer
  "sidebar.section.title": "LIBRARY & PLAYBACK",
  "sidebar.kivo.tagline.line1": "Kivo Music",
  "sidebar.kivo.tagline.line2":
    "Local · High performance · Apple-inspired",
  "sidebar.nav.library.title": "Library",
  "sidebar.nav.library.description":
    "Browse your local music by songs, albums or artists.",
  "sidebar.nav.playlists.title": "Playlists",
  "sidebar.nav.playlists.description":
    "Playlists and smart lists based on your listening history.",
  "sidebar.nav.nowPlaying.title": "Now Playing",
  "sidebar.nav.nowPlaying.description":
    "Large artwork view and detailed information.",
  "sidebar.nav.settings.title": "Settings",
  "sidebar.nav.settings.description":
    "Cover cache, logs and experimental features.",
  "sidebar.miniPlayer.badge.experimental": "Experimental",
  "sidebar.miniPlayer.title": "Mini player",
  "sidebar.section.libraryAndPlayback": "Library & Playback",

  // Library · grouped views (albums / artists)
  "library.groups.unknownArtistLabel": "Unknown artist",
  "library.groups.albums.empty": "No albums under the current filter.",
  "library.groups.albums.card.trackCount": "{count} tracks",
  "library.groups.albums.card.favoriteCount":
    " · {count} marked as favorite",
  "library.groups.artists.empty": "No artists under the current filter.",
  "library.groups.artists.header.artist": "Artist",
  "library.groups.artists.header.trackCount": "Tracks",
  "library.groups.artists.header.albumCount": "Albums",

  // Library · tracks view (virtual list)
    "library.tracks.fallbackTitle": "Unknown title",
  "library.tracks.fallbackArtist": "Unknown artist",
  "library.tracks.fallbackAlbum": "Unknown album",

  "library.tracks.favorite.tooltip.on": "Remove from favorites",
  "library.tracks.favorite.tooltip.off": "Mark as favorite",

  "library.tracks.emptyMessage":
    "Your library is empty. Import some local music files to get started.",

  // Library · track context menu
  "library.contextMenu.play": "Play this track",
  "library.contextMenu.playNext": "Play next",
  "library.contextMenu.appendToQueue": "Add to end of queue",
  "library.contextMenu.openInFolder": "Open containing folder",

  // Library · table headers (library table)
  "library.table.header.title": "Title",
  "library.table.header.artist": "Artist",
  "library.table.header.album": "Album",
  "library.table.header.duration": "Duration",
  "library.table.header.playCount": "Play count",
  "library.table.header.lastPlayed": "Last played",
  "library.table.header.actions": "Actions",

  // Playlists · tabs (top segmented control)
  "playlist.tabs.queue": "Current queue",
  "playlist.tabs.recentlyAdded": "Recently added",
  "playlist.tabs.recentlyPlayed": "Recently played",
  "playlist.tabs.mostPlayed": "Most played",
  "playlist.tabs.favorites": "Liked songs",

  // Sidebar · mini player card
  "sidebar.miniPlayer.description":
    "Mini mode: a compact playback window.",

  // Navigation · Playlists plural (reserved)
  "nav.playlists.label": "Playlists",
  "nav.playlists.description":
    "Playlists and smart lists, auto-updated from your play history.",

  // Playlists · table view (shared by queue / smart lists)
  "playlists.table.header.title": "Title",
  "playlists.table.header.artist": "Artist",
  "playlists.table.header.album": "Album",
  "playlists.table.header.playCount": "Plays",
  "playlists.table.header.lastPlayed": "Last played",
  "playlists.table.header.favorite": "Liked",
  "playlists.table.header.actions": "Actions",
    // Backward compatibility for old keys: playlist.table.actions.*
  "playlist.table.actions.playNext": "Play next",
  "playlist.table.actions.appendToQueue": "Add to queue",


  "playlists.table.row.action.playNext": "Play next",
  "playlists.table.row.action.appendToQueue": "Add to queue",

  "playlists.table.empty.currentTab": "No tracks in this list yet.",

  // Settings · page shell & tabs
  "settings.page.title": "Settings",
  "settings.page.sidebar.sectionLabel": "Sections",

  "settings.tabs.general": "General",
  "settings.tabs.coverCache": "Cover cache",
  "settings.tabs.developer": "Developer",

  // Settings · General · theme
  "settings.general.theme.title": "Theme",
  "settings.general.theme.subtitle":
    "Switch between light, dark and system. Only affects the UI appearance, not playback.",
  "settings.general.theme.currentLabel": "Current theme: ",
  "settings.general.theme.description":
    "Changing the theme only affects appearance, not playback or audio quality.",
  "settings.general.theme.currentValue.default": "Kivo default theme",

  // Settings · General · language
  "settings.general.language.title": "Interface language",
  "settings.general.language.subtitle":
    "Follow system language by default, or choose manually to test multi-language UI.",
  "settings.general.language.currentLabel": "Current language: ",
  "settings.general.language.note":
    "Language changes take effect immediately. Most UI text already uses i18n; a few experimental areas are still being completed.",

  // Settings · Cover cache · directory info
  "settings.cache.directory.title": "Cover cache directory",
  "settings.cache.directory.currentLabel": "Current effective directory:",
  "settings.cache.directory.loading": "Loading…",
  "settings.cache.directory.chooseButton": "Choose cover cache directory…",
  "settings.cache.directory.help":
    "By default Kivo uses an AppData folder for cover cache. You can move it to a larger drive (D:, E:, etc.). Default path:",

  // Settings · Cover cache · stats & actions
  "settings.cache.stats.title": "Cover cache stats & maintenance",
  "settings.cache.stats.loading": "Loading cache statistics…",
  "settings.cache.stats.fileCountLabel": "Cached files: ",
  "settings.cache.stats.sizeLabel": "Total size: ",
  "settings.cache.stats.trackEntriesLabel": "Track cover index entries: ",
  "settings.cache.stats.folderEntriesLabel": "Folder cover index entries: ",
  "settings.cache.stats.action.processing": "Processing…",
  "settings.cache.stats.action.refresh": "Refresh stats",
  "settings.cache.stats.action.repairIndex": "Repair cover index",
  "settings.cache.stats.action.clearCache": "Clear cover cache",

  // Settings · Cover cache · messages
  "settings.cache.message.refreshSuccess":
    "Cover cache stats have been refreshed.",
  "settings.cache.message.refreshFailure":
    "Failed to refresh cover cache stats. Please check the console logs.",
  "settings.cache.message.dirUnchanged":
    "Cover cache directory has not changed.",
  "settings.cache.message.dirUpdatedWithMigrate":
    "Cover cache directory updated and existing cache has been migrated.",
  "settings.cache.message.dirUpdatedWithoutMigrate":
    "Cover cache directory updated. Existing cache was not migrated.",
  "settings.cache.message.chooseDirError":
    "An error occurred while choosing the cover cache directory. Please check the console logs.",
  "settings.cache.message.clearSuccess": "Cover cache has been cleared.",
  "settings.cache.message.clearFailure":
    "Failed to clear cover cache. Please check the console logs.",
  "settings.cache.message.repairCompleted":
    "Cover index self-check completed:",
  "settings.cache.message.repairTrackSummary":
    "Checked {coverChecked} track cover index entries, removed {coverRemoved} broken records.",
  "settings.cache.message.repairFolderSummary":
    "Checked {folderChecked} folder cover index entries, removed {folderRemoved} broken records.",
  "settings.cache.message.repairFailure":
    "Failed to repair cover index. Please check the console logs.",

  // Settings · Cover cache · confirmation dialogs
  "settings.cache.dialog.chooseDirTitle": "Choose cover cache directory",
  "settings.cache.dialog.changeDirConfirm":
    "You are about to change the cover cache directory.\n\nDo you want to migrate existing cached covers to the new directory?\n\nIt is recommended to choose Yes so existing covers are preserved.\nChoose No to start with an empty cache and write new covers into the new directory.",
  "settings.cache.dialog.clearConfirm":
    "Are you sure you want to clear all cover cache?\n\nThis will NOT delete your music files or kivo-library.json.\nIt only deletes cached covers, which will be regenerated as needed.",
  "settings.cache.dialog.repairConfirm":
    "This will scan the cover index files (covers.json / folder-covers.json) and remove entries that point to non-existent files.\n\nIt will not delete any new cover files and will not affect your music library; it only cleans up index entries.\n\nDo you want to continue?",

  // Settings · Cover cache · debug notes for future maintainers
  "settings.cache.debug.title": "Debug notes (for future maintainers)",
  "settings.cache.debug.item1.prefix":
    "Cover cache directory and settings are stored under ",
  "settings.cache.debug.item1.suffix": ".",
  "settings.cache.debug.item2":
    " keeps the mapping from tracks to cached cover file paths.",
  "settings.cache.debug.item3.prefix":
    " keeps scan results for folders (including those without covers) so we do not repeatedly probe missing ",
  "settings.cache.debug.item3.suffix": " files.",
  "settings.cache.debug.item4.prefix":
    "As long as Now Playing and list views use ",
  "settings.cache.debug.item4.suffix":
    " to resolve covers, they will all share the same cache logic.",
      // Sidebar · brand & footer
  "sidebar.brand.tagline":
    "Local-first music player for people who still own their files.",
  "sidebar.footer.line1":
    "Kivo is under active development; feedback is welcome.",
  "sidebar.footer.line2":
    "This early build focuses on local playback and light-weight stats.",

  // Playlists · header legacy keys (aliases for existing messages)
  "playlists.header.smartListTip":
    "Smart lists update automatically based on your listening behavior.",
  "playlists.header.current.badgeNowPlaying": "Now playing",
  "playlists.header.actions.clearQueue": "Clear current queue",

  // Settings · Developer panel
  "settings.developer.title": "Developer settings (reserved)",
  "settings.developer.description.line1":
    "These options are currently stored only in localStorage and do not directly affect the playback core yet.",
  "settings.developer.description.line2":
    "In future versions they can be wired into log.ts, debug overlays and visualizer features.",

  "settings.developer.logLevel.label": "Log level (reserved)",
  "settings.developer.logLevel.option.debug": "Debug (most verbose)",
  "settings.developer.logLevel.option.info": "Info (default)",
  "settings.developer.logLevel.option.warn":
    "Warn (warnings + errors)",
  "settings.developer.logLevel.option.error":
    "Error (errors only)",
  "settings.developer.logLevel.note":
    "Later this can be connected to log.ts to control global logging output.",

  "settings.developer.overlay.label": "Debug overlay (reserved)",
  "settings.developer.overlay.note":
    "May show current playback state, memory usage and other realtime diagnostics in the UI later.",

  "settings.developer.visualizer.label":
    "Enable Web Audio spectrum visualizer (experimental)",
  "settings.developer.visualizer.note":
    "When enabled, the Now Playing page shows a simple bar spectrum visualizer which may slightly increase CPU usage.",

};
