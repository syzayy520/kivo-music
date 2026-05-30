import {
  fallbackArtworkKeys,
  fallbackArtworkSequence,
  fallbackArtworkUrls,
  type FallbackArtworkKey,
} from './fallbackArtworkKeys'

type ArtworkLike = {
  artworkKey?: string
  artworkUrl?: string
  coverUrl?: string
  coverPath?: string
  imageUrl?: string
}

export type ArtworkStyle = {
  backgroundImage: string
}

const fallbackArtworkKeySet = new Set<string>(Object.values(fallbackArtworkKeys))

export function resolveFallbackArtworkKey(item: ArtworkLike, index = 0): FallbackArtworkKey {
  if (item.artworkKey && fallbackArtworkKeySet.has(item.artworkKey)) {
    return item.artworkKey as FallbackArtworkKey
  }

  return fallbackArtworkSequence[index % fallbackArtworkSequence.length]
}

export function resolveRealArtworkUrl(item: ArtworkLike) {
  return item.artworkUrl || item.coverUrl || item.coverPath || item.imageUrl || ''
}

export function resolveFallbackArtworkUrl(item: ArtworkLike, index = 0) {
  const key = resolveFallbackArtworkKey(item, index)
  return fallbackArtworkUrls[key]
}

export function resolveArtworkStyle(item: ArtworkLike, index = 0): ArtworkStyle {
  const realUrl = resolveRealArtworkUrl(item)
  const fallbackUrl = resolveFallbackArtworkUrl(item, index)
  return {
    backgroundImage: `url(${realUrl || fallbackUrl})`,
  }
}
