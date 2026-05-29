import './albumDetail.css'
import '../../shared/styles/artwork.css'
import { AlbumHero } from './components/AlbumHero'
import { TrackList } from './components/TrackList'

export function AlbumDetail() {
  return (
    <>
      <AlbumHero />
      <TrackList />
    </>
  )
}
