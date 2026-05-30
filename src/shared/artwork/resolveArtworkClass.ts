import { fallbackArtworkKeys, fallbackArtworkSequence, type FallbackArtworkKey } from './fallbackArtworkKeys'

type ArtworkLike = {
  artworkKey?: string
  artworkUrl?: string
  coverUrl?: string
  coverPath?: string
  imageUrl?: string
  cover?: string
}

const fallbackArtworkKeySet = new Set<string>(Object.values(fallbackArtworkKeys))

export function hasRealArtwork(item: ArtworkLike) {
  return Boolean(item.artworkUrl || item.coverUrl || item.coverPath || item.imageUrl || item.cover)
}

export function resolveFallbackArtworkKey(item: ArtworkLike, index = 0): FallbackArtworkKey {
  if (item.artworkKey && fallbackArtworkKeySet.has(item.artworkKey)) {
    return item.artworkKey as FallbackArtworkKey
  }

  return fallbackArtworkSequence[index % fallbackArtworkSequence.length]
}

export function resolveArtworkClass(item: ArtworkLike, index = 0) {
  return `artwork-${resolveFallbackArtworkKey(item, index)}`
}
