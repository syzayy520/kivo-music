# Kivo Music Architecture

Kivo Music is a local-first desktop music player. The UI direction is informed by real Apple Music for Mac screenshots: light navigation weight, clear Home hierarchy, horizontal shelves, restrained controls, and a stable bottom player.

## Product direction

- Local music library first.
- Desktop player first.
- Album, artist, playlist, lyrics, queue, and playback surfaces must remain calm and maintainable.
- Audio quality information can be shown as local-library metadata, but UI must not become a technical dashboard.
- Do not introduce external resource, download, cloud drive, torrent, magnet, piracy, or resource-site semantics.

## Architecture rules

- One feature block owns one folder.
- One file owns one responsibility.
- Large features are split into data, copy, composer, components, and styles.
- App files compose surfaces only; they must not own business transformation logic.
- Mock data lives under `src/data/mock`.
- Shared types live under `src/shared/types`.
- Shared design primitives live under `src/shared/styles`.
- Feature-specific styles stay beside the feature.

## Current feature boundaries

```text
src/app
  App.tsx              App-level composition only
  appShell.css         App shell layout only

src/data/mock
  albums.ts            Album mock data only
  artists.ts           Artist mock data only
  playlists.ts         Playlist mock data only
  tracks.ts            Track mock data only

src/features/sidebar
  sidebarItems.ts      Sidebar navigation data only
  Sidebar.tsx          Sidebar rendering only
  sidebar.css          Sidebar styles only

src/features/top-bar
  topBarCopy.ts        Top bar copy only
  TopBar.tsx           Top bar rendering only
  topBar.css           Top bar styles only

src/features/listen-now
  listenNowCopy.ts     Listen Now copy only
  listenNowData.ts     Listen Now data boundary only
  ListenNow.tsx        Listen Now composition only
  components/*         Small visual sub-blocks
  listenNow.css        Listen Now styles only

src/features/album-detail
  albumDetailCopy.ts   Album detail copy only
  albumDetailData.ts   Album detail data boundary only
  AlbumDetail.tsx      Album detail composition only
  components/*         Hero and track-list sub-blocks
  albumDetail.css      Album detail styles only

src/features/library
  libraryCopy.ts       Library copy only
  libraryData.ts       Library data boundary only
  LibraryOverview.tsx  Library composition only
  components/*         Stat and shelf sub-blocks
  library.css          Library styles only

src/features/player
  playerState.ts       Playback mock state only
  playerData.ts        Player data boundary only
  PlayerBar.tsx        Player composition only
  components/*         Track, controls, meta sub-blocks
  playerBar.css        Player styles only

src/features/lyrics
  lyricsData.ts        Lyrics data boundary only
  LyricsPanel.tsx      Lyrics rendering only
  lyrics.css           Lyrics styles only

src/features/queue
  queueData.ts         Queue data boundary only
  QueuePreview.tsx     Queue rendering only
  queue.css            Queue styles only

src/features/now-playing
  NowPlaying.tsx       Lyrics and queue composition only
  nowPlaying.css       Now Playing styles only
```

## Apple Music reference principles

- Sidebar should be light and calm, not a heavy admin menu.
- The main surface should use strong headings and horizontal shelves.
- Cards should rely on cover art, spacing, and hierarchy rather than many buttons.
- The player should remain fixed, compact, and predictable.
- Lyrics and queue should support the playing context without becoming a second dashboard.

## When adding a feature

1. Create a feature folder.
2. Add copy in a copy file.
3. Add data adaptation in a data file.
4. Add one composer component.
5. Add small child components only as needed.
6. Add feature-local CSS.
7. Wire through `App.tsx` only after the feature boundary is clean.
