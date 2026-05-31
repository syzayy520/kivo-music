import type { TranslationKey } from '../../shared/i18n'

export type ListenNowShelfKey = 'recentlyPlayed' | 'recentlyAdded'

type ShelfCopy = {
  eyebrowKey: TranslationKey
  titleKey: TranslationKey
}

type TopPickCopy = {
  eyebrowKey: TranslationKey
  descriptionKey: TranslationKey
}

export const listenNowCopy: {
  showAllKey: TranslationKey
  topPicks: {
    eyebrowKey: TranslationKey
    titleKey: TranslationKey
    items: TopPickCopy[]
  }
  shelves: Record<ListenNowShelfKey, ShelfCopy>
} = {
  showAllKey: 'common.showAll',
  topPicks: {
    eyebrowKey: 'home.topPicks.eyebrow',
    titleKey: 'home.topPicks.title',
    items: [
      {
        eyebrowKey: 'home.topPicks.lateNight.eyebrow',
        descriptionKey: 'home.topPicks.lateNight.description',
      },
      {
        eyebrowKey: 'home.topPicks.focus.eyebrow',
        descriptionKey: 'home.topPicks.focus.description',
      },
      {
        eyebrowKey: 'home.topPicks.unwind.eyebrow',
        descriptionKey: 'home.topPicks.unwind.description',
      },
    ],
  },
  shelves: {
    recentlyPlayed: {
      eyebrowKey: 'home.shelves.recentlyPlayed.eyebrow',
      titleKey: 'home.shelves.recentlyPlayed.title',
    },
    recentlyAdded: {
      eyebrowKey: 'home.shelves.recentlyAdded.eyebrow',
      titleKey: 'home.shelves.recentlyAdded.title',
    },
  },
}
