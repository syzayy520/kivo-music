import type { PremiumLocalHomeIconName } from './PremiumLocalHomeIcon'

const artwork = {
  autumnSunset: '/artwork/fallback/autumn-sunset.webp',
  beamSky: '/artwork/fallback/beam-sky.webp',
  frequencyBlue: '/artwork/fallback/frequency-blue.webp',
  greenHills: '/artwork/fallback/green-hills.webp',
  moonPink: '/artwork/fallback/moon-pink.webp',
  oceanWave: '/artwork/fallback/ocean-wave.webp',
  pinkHill: '/artwork/fallback/pink-hill.webp',
  purpleCloud: '/artwork/fallback/purple-cloud.webp',
  springBloom: '/artwork/fallback/spring-bloom.webp',
  summerLake: '/artwork/fallback/summer-lake.webp',
  winterMist: '/artwork/fallback/winter-mist.webp',
} as const

export type PremiumLocalAlbumCard = {
  id: string
  title: string
  artist: string
  meta: string
  imageUrl: string
}

type PremiumLocalFeature = {
  id: string
  eyebrow: string
  title: string
  description: string
  imageUrl: string
  iconName: PremiumLocalHomeIconName
  tone: 'dark' | 'soft'
}

export const premiumLocalHomeData = {
  hero: {
    artist: 'Luna Vale',
    description: '深夜本地精选 · FLAC 24-bit',
    eyebrow: '继续听',
    imageUrl: artwork.pinkHill,
    primaryActionLabel: '继续播放',
    progressLabel: '上次播放到 2:37 / 5:48',
    progressPercent: 46,
    secondaryActionLabel: '加入队列',
    title: 'Midnight Archive',
  },
  features: [
    {
      description: '1,286 首 · FLAC / ALAC',
      eyebrow: '高品质收藏',
      iconName: 'sparkle',
      id: 'hi-res-collection',
      imageUrl: artwork.winterMist,
      title: 'Hi-Res Collection',
      tone: 'dark',
    },
    {
      description: '12 首 · 42 分钟',
      eyebrow: '最近完整专辑',
      iconName: 'album',
      id: 'recent-complete-album',
      imageUrl: artwork.greenHills,
      title: 'Echo Bloom',
      tone: 'soft',
    },
  ] satisfies PremiumLocalFeature[],
  recentlyAdded: [
    { artist: 'Orbit Room', id: 'recent-added-dawn-signals', imageUrl: artwork.autumnSunset, meta: 'Local · ALAC', title: 'Dawn Signals' },
    { artist: 'Tucker Wetmore', id: 'recent-added-what-not-to', imageUrl: artwork.winterMist, meta: 'Local · AAC', title: 'What Not To' },
    { artist: 'The Vernon Spring', id: 'recent-added-river-run', imageUrl: artwork.oceanWave, meta: 'Lossless · FLAC', title: 'River Run' },
    { artist: 'Yuno', id: 'recent-added-blest', imageUrl: artwork.springBloom, meta: 'Local · AAC', title: 'Blest' },
    { artist: 'Local Natives', id: 'recent-added-ultrablur', imageUrl: artwork.frequencyBlue, meta: 'Hi-Res · 96 kHz', title: 'Ultrablur Nights' },
    { artist: 'Nao', id: 'recent-added-jupiter', imageUrl: artwork.beamSky, meta: 'Local · AAC', title: 'Jupiter' },
  ] satisfies PremiumLocalAlbumCard[],
  recentlyPlayed: [
    { artist: 'Luna Vale', id: 'recent-played-midnight-archive', imageUrl: artwork.pinkHill, meta: 'Lossless · FLAC · 24-bit', title: 'Midnight Archive' },
    { artist: 'Nova District', id: 'recent-played-static-gardens', imageUrl: artwork.summerLake, meta: 'Local · AAC', title: 'Static Gardens' },
    { artist: 'Solstice', id: 'recent-played-golden-hour', imageUrl: artwork.beamSky, meta: 'Local · ALAC', title: 'Golden Hour' },
    { artist: 'Luna Vale', id: 'recent-played-afterglow', imageUrl: artwork.moonPink, meta: 'Local · AAC', title: 'Afterglow' },
    { artist: 'Orbit Room', id: 'recent-played-velvet-lights', imageUrl: artwork.purpleCloud, meta: 'Lossless · FLAC', title: 'Velvet Lights' },
    { artist: 'Yuno', id: 'recent-played-paper-planes', imageUrl: artwork.winterMist, meta: 'Local · AAC', title: 'Paper Planes' },
  ] satisfies PremiumLocalAlbumCard[],
  player: {
    artist: 'Luna Vale',
    currentTime: '2:37',
    duration: '5:48',
    imageUrl: artwork.pinkHill,
    progressPercent: 46,
    quality: 'FLAC 24-bit / 96 kHz',
    title: 'Midnight Archive',
    volumePercent: 68,
  },
} as const
