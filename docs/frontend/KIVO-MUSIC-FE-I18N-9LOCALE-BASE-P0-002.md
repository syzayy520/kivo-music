# KIVO-MUSIC-FE-I18N-9LOCALE-BASE-P0-002 Completion Audit

## State Header

- Repo: `syzayy520/kivo-music`
- Branch: `v1`
- Scope: frontend i18n foundation and existing visible copy wiring
- Base ticket dependency: `KIVO-MUSIC-FE-BASELINE-FREEZE-P0-001`

## Goal

Add a lightweight 9-locale i18n foundation and wire the currently visible Home, Sidebar, TopBar, and PlayerBar copy without changing the frozen Home visual baseline.

## Apple Reference Before Work

Apple Music reference areas used for this ticket:

1. Home sidebar and first-screen hierarchy.
2. Home shelves and hero cards.
3. Global player bar accessibility surface.

What Kivo borrowed:

1. Stable navigation naming boundaries.
2. Home shelf copy hierarchy.
3. Player control accessibility copy.

What Kivo did not copy:

1. No online catalog language.
2. No broad Home redesign.
3. No PlayerBar idle-state behavior change.

## Implemented Boundaries

### i18n core

- `src/shared/i18n/localeTypes.ts`
- `src/shared/i18n/detectLocale.ts`
- `src/shared/i18n/index.ts`

Responsibilities:

- locale type definitions
- supported-locale detection
- dictionary registration
- typed translation lookup
- token interpolation compatible with the current TypeScript target

### locale dictionaries

- `src/shared/i18n/locales/en-US.ts`
- `src/shared/i18n/locales/zh-CN.ts`
- `src/shared/i18n/locales/zh-TW.ts`
- `src/shared/i18n/locales/ja-JP.ts`
- `src/shared/i18n/locales/ko-KR.ts`
- `src/shared/i18n/locales/de-DE.ts`
- `src/shared/i18n/locales/fr-FR.ts`
- `src/shared/i18n/locales/es-ES.ts`
- `src/shared/i18n/locales/pt-BR.ts`

Supported locales:

1. Simplified Chinese
2. Traditional Chinese
3. English
4. Japanese
5. Korean
6. German
7. French
8. Spanish
9. Portuguese Brazil

### wired feature surfaces

- `src/features/sidebar/sidebarItems.ts`
- `src/features/sidebar/Sidebar.tsx`
- `src/features/top-bar/topBarCopy.ts`
- `src/features/top-bar/TopBar.tsx`
- `src/features/listen-now/listenNowCopy.ts`
- `src/features/listen-now/components/TopPicksShelf.tsx`
- `src/features/listen-now/components/AlbumShelf.tsx`
- `src/features/listen-now/components/ListenNowHero.tsx`
- `src/features/player/playerBarControls.ts`
- `src/features/player/PlayerBar.tsx`
- `src/features/player/components/PlayerControls.tsx`
- `src/features/player/components/PlayerMeta.tsx`
- `src/features/player/components/PlayerTrackInfo.tsx`

## Engineering Review

- Modules remain single responsibility.
- Copy keys live in copy/model files, not pages.
- Render components call `t()` only for display text and aria labels.
- Page composition was not expanded.
- No CSS or layout redesign was introduced.
- No backend audio logic was touched by this frontend ticket.
- PlayerBar idle hidden behavior remains unchanged.

## Build Issues Addressed

The reported local TypeScript errors were addressed:

1. Removed use of removed `heroEyebrow`, `heroTitle`, and `heroDescription` fields from `ListenNowHero.tsx`.
2. Replaced `String.prototype.replaceAll` with `split().join()` token replacement to stay compatible with the current TypeScript library target.
3. Added missing `home.hero.play` and `home.hero.viewAlbum` keys across all 9 locale dictionaries.

## Remaining Validation For Local Machine

Run from the repository root:

```powershell
npm run build
git diff --stat
git status --short
```

Expected result:

- TypeScript should no longer report the old `ListenNowHero` copy-field errors.
- TypeScript should no longer report the old `replaceAll` library-target error.
- `git status --short` should be clean after pulling `origin/v1`.

## Post-Implementation Apple Comparison

Apple reference comparison:

1. Sidebar text remains a stable navigation system, matching Apple Music's calm navigation hierarchy.
2. Home shelf and hero copy now use typed keys while preserving the existing Kivo visual rhythm.
3. PlayerBar copy and aria labels are localized without changing the player surface.

Kivo-specific preservation:

1. Kivo remains a local-first desktop music player.
2. Home baseline is preserved.
3. Fallback artwork remains acceptable at this stage.
4. PlayerBar remains hidden on startup/idle.
5. No temporary UI text was intentionally added.
6. No external-content discovery language was introduced.

## Completion Decision

P0-002 is code-complete pending local gate confirmation.
