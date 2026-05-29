# Kivo Music

Kivo Music is a flagship desktop-first local music player built around premium visual order, local music ownership, lyrics-first listening, queue/search foundations, and a future Kivo Playback Core.

## Run

```bash
npm install
npm run dev
```

## Current state

This repository now contains the React/Vite foundation for the Kivo Music product direction:

- App shell, sidebar, top bar, Listen Now, album detail, lyrics, queue, settings, library overview, and player bar.
- Feature-first folder boundaries.
- Typed local music data model.
- Mock data shaped like future real data.
- Design tokens and accessibility baseline.
- Kivo Playback Core boundary documentation.

## Non-negotiable rules

- One feature, one folder.
- One folder, one responsibility.
- One file, one responsibility.
- Pages only compose and wire.
- Mock data, copy, types, state, and styles stay separated.
- No temporary architecture.
- No oversized page dumping.
