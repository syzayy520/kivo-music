export const fallbackArtworkKeys = {
  pinkHill: 'pink-hill',
  purpleCloud: 'purple-cloud',
  greenHills: 'green-hills',
  summerLake: 'summer-lake',
  moonPink: 'moon-pink',
  beamSky: 'beam-sky',
  springBloom: 'spring-bloom',
  autumnSunset: 'autumn-sunset',
  winterMist: 'winter-mist',
  oceanWave: 'ocean-wave',
  paperPlane: 'paper-plane',
  frequencyBlue: 'frequency-blue',
} as const

export type FallbackArtworkKey = (typeof fallbackArtworkKeys)[keyof typeof fallbackArtworkKeys]

export const fallbackArtworkUrls: Record<FallbackArtworkKey, string> = {
  'pink-hill': '/artwork/fallback/pink-hill.webp',
  'purple-cloud': '/artwork/fallback/purple-cloud.webp',
  'green-hills': '/artwork/fallback/green-hills.webp',
  'summer-lake': '/artwork/fallback/summer-lake.webp',
  'moon-pink': '/artwork/fallback/moon-pink.webp',
  'beam-sky': '/artwork/fallback/beam-sky.webp',
  'spring-bloom': '/artwork/fallback/spring-bloom.webp',
  'autumn-sunset': '/artwork/fallback/autumn-sunset.webp',
  'winter-mist': '/artwork/fallback/winter-mist.webp',
  'ocean-wave': '/artwork/fallback/ocean-wave.webp',
  'paper-plane': '/artwork/fallback/paper-plane.webp',
  'frequency-blue': '/artwork/fallback/frequency-blue.webp',
}

export const fallbackArtworkSequence: FallbackArtworkKey[] = [
  fallbackArtworkKeys.pinkHill,
  fallbackArtworkKeys.purpleCloud,
  fallbackArtworkKeys.greenHills,
  fallbackArtworkKeys.summerLake,
  fallbackArtworkKeys.moonPink,
  fallbackArtworkKeys.beamSky,
  fallbackArtworkKeys.springBloom,
  fallbackArtworkKeys.autumnSunset,
  fallbackArtworkKeys.winterMist,
  fallbackArtworkKeys.oceanWave,
  fallbackArtworkKeys.paperPlane,
  fallbackArtworkKeys.frequencyBlue,
]
