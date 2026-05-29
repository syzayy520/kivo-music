export type SearchScope = 'all' | 'tracks' | 'albums' | 'artists' | 'playlists' | 'lyrics'

export type SearchState = {
  query: string
  scope: SearchScope
  recentSearches: string[]
  filters: {
    year?: number
    genre?: string
    quality?: string
    rating?: number
  }
}
