import { ClassicHomeExperience } from './experiences/classic/ClassicHomeExperience'
import { PremiumLocalHomeExperience } from './experiences/premiumLocal/PremiumLocalHomeExperience'
import type { HomeExperienceDefinition, HomeExperienceId } from './homeExperienceTypes'

export const DEFAULT_HOME_EXPERIENCE_ID: HomeExperienceId = 'classic'
export const ACTIVE_HOME_EXPERIENCE_ID: HomeExperienceId = 'premiumLocal'

export const HOME_EXPERIENCES: HomeExperienceDefinition[] = [
  {
    id: 'classic',
    component: ClassicHomeExperience,
  },
  {
    id: 'premiumLocal',
    component: PremiumLocalHomeExperience,
  },
]

export function resolveHomeExperience(id: HomeExperienceId = ACTIVE_HOME_EXPERIENCE_ID) {
  const fallbackExperience = HOME_EXPERIENCES.find(
    (experience) => experience.id === DEFAULT_HOME_EXPERIENCE_ID,
  )

  return HOME_EXPERIENCES.find((experience) => experience.id === id) ?? fallbackExperience
}
