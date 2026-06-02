import type { ComponentType } from 'react'
import type { HomeExperienceShellMode } from './homeExperienceShellMode'

export type HomeExperienceId = 'classic' | 'premiumLocal'

export type HomeExperienceDefinition = {
  id: HomeExperienceId
  shellMode: HomeExperienceShellMode
  component: ComponentType
}
