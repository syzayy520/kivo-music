import type { ComponentType } from 'react'

export type HomeExperienceId = 'classic' | 'premiumLocal'

export type HomeExperienceDefinition = {
  id: HomeExperienceId
  component: ComponentType
}
