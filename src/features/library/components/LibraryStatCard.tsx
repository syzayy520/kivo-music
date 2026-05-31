import { libraryStatCardStyle } from './libraryStatCardStyle'
import './libraryStatCard.css'

type LibraryStatCardProps = {
  label: string
  value: string
}

export function LibraryStatCard({ label, value }: LibraryStatCardProps) {
  return (
    <article className="km-library-stat" style={libraryStatCardStyle}>
      <strong>{value}</strong>
      <span>{label}</span>
    </article>
  )
}
