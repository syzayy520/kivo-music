# Kivo Music

A flagship desktop-first music player preview.

This repository is now using `syzayy520/kivo-music` as the main Kivo Music workspace.

## Current build

The current commit contains a polished static product preview for the Kivo Music visual direction. It intentionally avoids a JavaScript entry so the preview can open directly in a browser while the app architecture is prepared.

## Product direction

Kivo Music is not an Apple Music clone. It is an original premium local music player with:

- Apple Music-level visual order.
- Local music ownership as the product center.
- Album-first library presentation.
- Lyrics, queue, search, ratings, and audio-quality foundations.
- A future Kivo Playback Core boundary rather than temporary playback wiring.

## Engineering rules

- One feature, one folder.
- One folder, one responsibility.
- One file, one responsibility.
- Pages only compose and wire.
- Mock data, copy, types, state, and styles stay separated.
- No oversized page dumping.
- No temporary architecture.
