type LibraryStatCardProps = {
  label: string
  value: string
}

export function LibraryStatCard({ label, value }: LibraryStatCardProps) {
  return (
    <article className="km-library-stat">
      <strong>{value}</strong>
      <span>{label}</span>
    </article>
  )
}
