# Kivo Music Architecture

Kivo Music is a feature-first, desktop-first application.

## Rules

1. One feature owns one folder.
2. One folder owns one responsibility.
3. One file owns one responsibility.
4. Pages only compose and wire features.
5. Mock data never lives inside UI components.
6. Copy never scatters across UI components.
7. Playback UI never talks directly to a backend engine.
8. Lyrics, search, queue, player, library, settings, and playback core are separate features.
9. No temporary architecture.
10. No oversized files.

## Feature map

- app-shell: desktop frame.
- sidebar: navigation.
- top-bar: title, search entry, and status.
- listen-now: local-library home.
- album-detail: album presentation and track list.
- player: visible player bar UI only.
- playback-core: Kivo Playback API boundary.
- queue: queue drawer and queue state.
- lyrics: lyric display and lyric state.
- search: search entry, results, scopes, filters, and ranking.
- library: local collection surfaces.
- settings: grouped settings shell.
- accessibility: contrast, focus, motion, and readable text.
