# Kivo Music Quality Gates

These gates protect maintainability and visual direction. They must be checked before every handoff.

## Non-negotiable rules

- Do not create large mixed-responsibility files.
- Do not put feature data, copy, rendering, and styles into one file.
- Do not put business transformation logic into `App.tsx`.
- Do not use global CSS as a dumping ground.
- Do not add external resource, download, cloud drive, torrent, magnet, piracy, or resource-site semantics.
- Do not claim local build success unless the build was actually run.

## File responsibility gate

For every changed file, answer:

```text
What is this file's single responsibility?
Did this change add unrelated responsibility?
Should this be split into a child file?
```

If the answer is unclear, stop and split the file.

## Feature boundary gate

A feature should usually follow this shape:

```text
feature-name/
  featureCopy.ts
  featureData.ts
  FeatureName.tsx
  components/
    SmallPart.tsx
  feature.css
```

The composer component may import child components, copy, data, and CSS. Child components should stay small and visual.

## CSS gate

- Feature CSS belongs inside the feature folder.
- Shared tokens belong in `src/shared/styles`.
- If one CSS file keeps growing, split by visual sub-block.
- Prefer calm Apple Music-like spacing, hierarchy, and restraint over decorative effects.

## Apple Music visual gate

Before visual changes, review real Apple Music for Mac screenshots and check:

```text
Is the sidebar light and calm?
Is the title hierarchy clear?
Are shelves horizontal and stable?
Are cards simple and cover-art-led?
Are buttons minimal?
Is the player compact and predictable?
Is the page avoiding dashboard clutter?
```

## Handoff checklist

Every handoff must include:

```text
Changed files
Single responsibility per changed file
Apple Music reference point
What was intentionally not changed
Latest commit SHA
Known limitations
```

## Known current limitation

`index.html` still needs to be added in a separate, focused task so the Vite entry can load in a browser. This must not be hidden inside another feature task.
