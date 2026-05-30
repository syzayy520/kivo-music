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
  'pink-hill': '/artwork/fallback/pink-hill.svg',
  'purple-cloud': '/artwork/fallback/purple-cloud.svg',
  'green-hills': '/artwork/fallback/green-hills.svg',
  'summer-lake': '/artwork/fallback/summer-lake.svg',
  'moon-pink': '/artwork/fallback/moon-pink.svg',
  'beam-sky': '/artwork/fallback/beam-sky.svg',
  'spring-bloom': '/artwork/fallback/spring-bloom.svg',
  'autumn-sunset': '/artwork/fallback/autumn-sunset.svg',
  'winter-mist': '/artwork/fallback/winter-mist.svg',
  'ocean-wave': '/artwork/fallback/ocean-wave.svg',
  'paper-plane': '/artwork/fallback/paper-plane.svg',
  'frequency-blue': '/artwork/fallback/frequency-blue.svg',
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
