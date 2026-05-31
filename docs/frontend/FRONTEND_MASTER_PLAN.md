# Kivo Music Frontend Master Plan v0.9 Final

Ticket baseline: `KIVO-MUSIC-FE-BASELINE-FREEZE-P0-001`

This document freezes the product-grade frontend rules for Kivo Music. Future frontend tickets must follow this baseline. Every ticket must land as a complete product-grade responsibility block.

## Current frontend baseline

- `src/main.tsx` mounts `App` and imports shared style foundations.
- `src/app/App.tsx` composes `WindowChrome`, `Sidebar`, `TopBar`, `ListenNow`, and `PlayerBar`.
- `src/features/listen-now/ListenNow.tsx` renders Top Picks, Recently Played, and Recently Added shelves.
- `src/features/player/playerBarVisibility.ts` hides the player bar when `sessionState === 'none'`.
- `src/features/player/playerState.ts` currently uses `sessionState: 'none'`, so the initial player bar is intentionally hidden.

## Home baseline freeze

The current Home surface is the visual baseline. Preserve:

1. Restrained left sidebar.
2. Clean Home title and first-screen breathing room.
3. Top search surface and Local Library status pill.
4. Top Picks horizontal hero-card rhythm.
5. Recently Played and Recently Added shelf structure.
6. Unified premium fallback artwork direction.
7. Idle-state PlayerBar hidden behavior.

Allowed hardening:

1. Move copy into 9-locale i18n.
2. Split Home into single-responsibility modules.
3. Route raw data through adapters and view models.
4. Formalize fallback artwork.
5. Add Storage Sources model boundaries.
6. Make Library Status a typed state boundary.
7. Add PlayerBar safe area for active playback.

Forbidden for Home baseline tickets:

1. No broad Home redesign.
2. Do not force the PlayerBar to appear in idle/startup state.
3. Do not fetch external artwork.
4. Do not add icons only to mimic another product.
5. Do not turn Recently Added into online content semantics.

## Highest engineering rules

1. Every module has one responsibility.
2. Files must stay small and maintainable.
3. Pages only compose.
4. A Page may compose layout, call feature components, pass view models, and hold minimal page-level UI state only.
5. A Page must not sort, filter, transform mock data, directly invoke backend commands, or contain large business decisions.
6. Features must be self-contained: `components`, `model`, `adapter`, `copy`, and `css` belong with the feature unless genuinely shared.
7. Raw data, mock data, and backend data must flow through an adapter before reaching UI components.
8. UI receives ViewModels, not raw payloads.
9. CSS must be layered: token, app shell, feature, then component.
10. Every new string must use a module-scoped i18n key.
11. UI can model future capabilities, but must not imply unfinished capabilities are complete.
12. One ticket equals one complete responsibility block. No drive-by refactors.

## i18n baseline

The first i18n baseline must support 9 locales:

- `zh-CN`
- `zh-TW`
- `en-US`
- `ja-JP`
- `ko-KR`
- `de-DE`
- `fr-FR`
- `es-ES`
- `pt-BR`

Key namespaces:

- `common.*`
- `sidebar.*`
- `topBar.*`
- `home.*`
- `albums.*`
- `recentlyAdded.*`
- `storageSources.*`
- `player.*`
- `queue.*`
- `settings.*`
- `emptyState.*`

Rules:

1. Fallback to `en-US`.
2. Default to system/browser locale until Settings owns manual switching.
3. Do not put every string into `common`.
4. Do not duplicate keys for the same concept.
5. Do not postpone translation until after pages are built.

## Storage Sources boundary

Kivo Music may plan and later support user-owned music storage:

- Local folder
- External drive
- NAS
- SMB
- LAN device
- WebDAV
- User-authorized cloud drive

Correct product language:

- Storage Sources
- Library Sources
- Connect your music storage
- Add NAS / Network Folder
- Add Cloud Drive
- Scan authorized library

All cloud/NAS/WebDAV integrations must mean user-owned or user-authorized music library sources. They must not become a public content discovery feature.

## Apple Music reference rule

Every UI/UX/page/component ticket must perform Apple Music reference comparison before and after implementation.

Before work:

1. Identify the matching Apple Music screenshot(s).
2. List Apple's functional blocks for this area.
3. List what Kivo already has.
4. List what Kivo should borrow.
5. List what Kivo must not copy.
6. State exact scope and non-goals.

After work:

1. Provide an implementation screenshot.
2. Compare block by block with the matching Apple reference.
3. Confirm Kivo remains a local-first desktop music player.
4. Confirm the Home baseline was not broken.
5. Confirm no temporary UI or `placeholder` text remains.
6. Confirm module single responsibility, small files, and page composition rules were preserved.
7. Give a pass/fail decision and required rework if any.

No Apple before/after analysis means the UI ticket is not complete.

## Apple Music mapping

Home:

- Borrow: today-to-listen structure, horizontal shelves, Recently Played, calm first-screen rhythm.
- Preserve Kivo difference: PlayerBar hidden on startup/idle is correct.

Albums:

- Borrow: cover wall, page-level search, restrained sorting, clear title/artist hierarchy.
- Preserve Kivo difference: source badges and fallback artwork must support local/NAS/cloud later.

Recently Added:

- Borrow: hero section, light Latest Songs list, responsive multi-column density.
- Preserve Kivo difference: Kivo means latest in the user's library, not online new releases.

PlayerBar:

- Borrow: floating glass capsule, stable global control center, safe area.
- Preserve Kivo difference: idle/startup hidden behavior remains.

Now Playing / Lyrics:

- Borrow: immersive backdrop, large lyric stage, minimal controls.
- Preserve Kivo difference: wait for Player UI State Contract; do not fake real playback or lyrics sync.

Sidebar / Navigation:

- Borrow: stable navigation, grouped hierarchy, light active state.
- Preserve Kivo difference: no online-platform direction.

Mini player / multi-window:

- Borrow later: one playback state powering multiple UI forms.
- Current phase: plan only, do not implement.

## Final ticket order

1. `KIVO-MUSIC-FE-BASELINE-FREEZE-P0-001` — freeze this baseline.
2. `KIVO-MUSIC-FE-I18N-9LOCALE-BASE-P0-002` — add 9-locale i18n foundation.
3. `KIVO-MUSIC-FE-DESIGN-TOKENS-P0-003` — stabilize tokens and safe area.
4. `KIVO-MUSIC-FE-FALLBACK-ARTWORK-SYSTEM-P0-004` — formalize fallback artwork.
5. `KIVO-MUSIC-FE-UI-INTERACTION-FOUNDATION-P0-005` — focus, hover, pressed, keyboard, aria.
6. `KIVO-MUSIC-FE-STATE-VIEWS-FOUNDATION-P0-006` — Empty, Loading, Error, StatusBadge.
7. `KIVO-MUSIC-FE-WINDOW-RESPONSIVE-RULES-P0-007` — responsive window rules.
8. `KIVO-MUSIC-FE-STORAGE-SOURCES-MODEL-P0-008` — frontend-only storage source model.
9. `KIVO-MUSIC-FE-HOME-MODULE-BOUNDARY-P0-009` — split Home into product modules.
10. `KIVO-MUSIC-FE-NAV-SYSTEM-P0-010` — product-grade navigation system.
11. `KIVO-MUSIC-FE-ALBUMS-PRODUCT-P1-011` — Albums cover-wall page.
12. `KIVO-MUSIC-FE-RECENTLY-ADDED-PRODUCT-P1-012` — Recently Added product page.
13. `KIVO-MUSIC-FE-FOLDERS-SOURCES-P1-013` — Folders / Sources product page.
14. `KIVO-MUSIC-FE-SETTINGS-SHELL-P1-014` — Settings shell.
15. `KIVO-MUSIC-FE-PLAYER-STATE-CONTRACT-P1-015` — Player UI state contract.
16. `KIVO-MUSIC-FE-QUEUE-PRODUCT-P1-016` — Queue product page.
17. `KIVO-MUSIC-FE-NOW-PLAYING-PRODUCT-P1-017` — Now Playing immersive page.

## P2 backlog

Do not start these in the current phase:

- Album Detail
- Artists product page
- Songs large-library table
- Playlists product page
- Mini Player
- Floating Lyrics
- Tray Controls
- Multi-window experience
- Real Storage Sources connection
- Real cover cache integration
- Real playback state integration

## Per-ticket delivery requirements

Every ticket must include:

1. State Header: repo, branch, base commit.
2. Apple Music before-work reference comparison.
3. Allowed Files.
4. Forbidden Files.
5. Non-goals.
6. Engineering rules confirmation.
7. Product-grade acceptance criteria.
8. Gates:
   - `npm run build`
   - `git diff --stat`
   - `git status --short`
9. UI screenshot when visual UI changes are made.
10. Apple Music post-work comparison when visual UI changes are made.
11. Completion report:
   - commit hash
   - changed files
   - diff stat
   - gates output
   - Home baseline preserved or not
   - forbidden semantics introduced or not
   - temporary UI / placeholder introduced or not

## Pull policy

A user pull should only be requested after a complete ticket is finished. Do not ask the user to pull for every tiny sub-step.