type PremiumLocalHomeIconProps = {
  label: string
}

export function PremiumLocalHomeIcon({ label }: PremiumLocalHomeIconProps) {
  return <span className="premium-local-home__icon" aria-hidden="true">{label}</span>
}

