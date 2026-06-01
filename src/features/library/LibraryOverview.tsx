import '../../shared/styles/artwork.css'
import { t } from '../../shared/i18n'
import { libraryCopy } from './libraryCopy'
import { libraryData } from './libraryData'
import { LibraryStatCard } from './components/LibraryStatCard'
import { RecentlyAddedShelf } from './components/RecentlyAddedShelf'
import { libraryStyle } from './libraryStyle'
import './library.css'

export function LibraryOverview() {
  return (
    <section className="km-library-overview" id="library" style={libraryStyle}>
      <div className="km-library-head">
        <p>{t(libraryCopy.eyebrow)}</p>
        <h2>{t(libraryCopy.title)}</h2>
      </div>
      <div className="km-library-stats" aria-label={t(libraryCopy.statsTitle)}>
        {libraryData.stats.map((stat) => (
          <LibraryStatCard label={stat.label} value={stat.value} key={stat.label} />
        ))}
      </div>
      <div className="km-library-shelf-head">
        <h3>{t(libraryCopy.recentlyAddedTitle)}</h3>
      </div>
      <RecentlyAddedShelf />
    </section>
  )
}
