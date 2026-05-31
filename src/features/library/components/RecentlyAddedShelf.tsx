import { libraryData } from '../libraryData'
import { recentlyAddedStyle } from './recentlyAddedStyle'
import './recentlyAddedShelf.css'

export function RecentlyAddedShelf() {
  return (
    <div className="km-recently-added" style={recentlyAddedStyle}>
      {libraryData.recentlyAddedAlbums.map((album) => (
        <article className="km-recently-added-card" key={album.id}>
          <div className={`km-recently-added-art artwork-${album.artworkKey}`} />
          <strong>{album.title}</strong>
          <span>{album.qualityLabel}</span>
        </article>
      ))}
    </div>
  )
}
