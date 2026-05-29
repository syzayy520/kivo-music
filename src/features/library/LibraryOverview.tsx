import '../../shared/styles/artwork.css'
import { libraryCopy } from './libraryCopy'
import { libraryData } from './libraryData'
import { LibraryStatCard } from './components/LibraryStatCard'
import { RecentlyAddedShelf } from './components/RecentlyAddedShelf'
import './library.css'

export function LibraryOverview() {
  return (
    <section className="km-library-overview" id="library">
      <div className="km-library-head">
        <p>{libraryCopy.eyebrow}</p>
        <h2>{libraryCopy.title}</h2>
      </div>
      <div className="km-library-stats" aria-label={libraryCopy.statsTitle}>
        {libraryData.stats.map((stat) => (
          <LibraryStatCard label={stat.label} value={stat.value} key={stat.label} />
        ))}
      </div>
      <div className="km-library-shelf-head">
        <h3>{libraryCopy.recentlyAddedTitle}</h3>
      </div>
      <RecentlyAddedShelf />
    </section>
  )
}
