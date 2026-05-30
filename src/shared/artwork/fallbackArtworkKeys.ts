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
