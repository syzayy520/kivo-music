import { ClassicHomeExperience } from './experiences/classic/ClassicHomeExperience'
import { PremiumLocalHomeExperience } from './experiences/premiumLocal/PremiumLocalHomeExperience'
import { resolveHomeExperienceShellMode } from './homeExperienceShellMode'
import type { HomeExperienceDefinition, HomeExperienceId } from './homeExperienceTypes'

export const DEFAULT_HOME_EXPERIENCE_ID: HomeExperienceId = 'classic'
export const ACTIVE_HOME_EXPERIENCE_ID: HomeExperienceId = 'premiumLocal'

export const HOME_EXPERIENCES: HomeExperienceDefinition[] = [
  {
    id: 'classic',
    shellMode: 'classic',
    component: ClassicHomeExperience,
  },
  {
    id: 'premiumLocal',
    shellMode: 'premiumLocal',
    component: PremiumLocalHomeExperience,
  },
]

export const ACTIVE_HOME_EXPERIENCE_SHELL_MODE = resolveHomeExperienceShellMode(
  HOME_EXPERIENCES,
  ACTIVE_HOME_EXPERIENCE_ID,
)

export function resolveHomeExperience(id: HomeExperienceId = ACTIVE_HOME_EXPERIENCE_ID) {
  const fallbackExperience = HOME_EXPERIENCES.find(
    (experience) => experience.id === DEFAULT_HOME_EXPERIENCE_ID,
  )

  return HOME_EXPERIENCES.find((experience) => experience.id === id) ?? fallbackExperience
}
