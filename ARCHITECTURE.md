# Kivo Music Architecture

Kivo Music is designed as a flagship player from day one. Implementation can move in phases, but architecture must not be temporary.

## Feature boundaries

- `app-shell`: desktop frame and layout only.
- `sidebar`: navigation model and navigation UI.
- `top-bar`: page title, search entry, and lightweight status.
- `listen-now`: local-library home experience.
- `album-detail`: album hero, album metadata, actions, and track list.
- `player`: visible player bar UI.
- `playback-core`: final Kivo Playback API boundary.
- `queue`: queue drawer and queue state.
- `lyrics`: lyrics display and lyric state.
- `search`: search scopes, filters, results, and ranking.
- `library`: local collection surfaces.
- `settings`: grouped settings shell.
- `accessibility`: focus, contrast, motion, readable text.

## Playback core rule

Kivo Playback Core is the product playback boundary. Backend technology is an implementation detail. UI must only use Kivo playback commands, state, events, errors, and diagnostics.

The long-term architecture must support queue, gapless, ReplayGain, crossfade, output devices, exclusive output, bit-perfect diagnostics, lyrics sync, playback history, and scrobbling boundaries.
