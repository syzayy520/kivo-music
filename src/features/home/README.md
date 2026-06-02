# Home Experience Guardrails

This document records the Home Experience boundary for Kivo Music. It is intentionally scoped to the Home feature so future work can refine the Home surface without breaking the App Shell, the classic home wrapper, or the playback pipeline.

## 1. Home Experience architecture

- `HomeExperienceHost` is the Home entry point.
- `homeExperienceRegistry` owns Home Experience registration and active experience selection.
- `classic` is the legacy Home wrapper. It renders the existing `ListenNow` experience and must remain available as a safe fallback.
- `premiumLocal` is the second real App Home Experience for Kivo Music.
- `premiumLocal` is not a web demo, not a landing page, and not a visual skin applied over `classic`.

## 2. Current default state

- The active Home Experience is currently `premiumLocal`.
- `classic` is still registered and must not be deleted.
- To temporarily fall back to `classic`, only change the active id in `homeExperienceRegistry` back to `classic`.
- Do not remove `classic` while `premiumLocal` is being refined.

## 3. File boundaries

### classic

- `src/features/home/experiences/classic/**`
- `src/features/listen-now/**`

### premiumLocal

- `src/features/home/experiences/premiumLocal/**`

### Host / Registry

- `src/features/home/HomeExperienceHost.tsx`
- `src/features/home/homeExperienceRegistry.ts`

## 4. No-go areas

Future `premiumLocal` refinement must not directly modify:

- `src/app/App.tsx`
- `src/features/listen-now/**`
- `src/features/sidebar/**`
- `src/features/top-bar/**`
- `src/features/player/**`
- `src-tauri/**`
- package files and lock files
- `index.html`

## 5. Design principles

- Kivo Music is a desktop player and must stay window-first.
- Do not turn Home into a browser-style full-screen landing page.
- The 1280 / 1366 / 1440 / 1600 desktop window widths must remain stable and visually balanced.
- Home content must not stretch infinitely across ultra-wide displays.
- Sidebar, TopBar, and PlayerBar are App Shell surfaces. They do not belong to an individual Home Experience and must not be casually rewritten inside Home tickets.
- `premiumLocal` must keep the independent `.km-home-premium-*` class prefix.
- `premiumLocal` must not pollute or override `classic` styles.

## 6. Follow-up refinement guidance

- Refine only `premiumLocal` internals first: Hero, Shelf, and PlayerSurface.
- Do not combine Home Experience work with App Shell changes in the same ticket.
- If Shell coordination is needed later, open a separate ticket and verify that `classic` remains unaffected.
- If a Settings-page Home Experience switch is needed later, open a separate ticket for that workflow.
