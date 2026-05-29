import { libraryData } from '../libraryData'

export function RecentlyAddedShelf() {
  return (
    <div className="km-recently-added">
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
