import { ClassicHomeExperience } from './experiences/classic/ClassicHomeExperience'
import type { HomeExperienceDefinition, HomeExperienceId } from './homeExperienceTypes'

export const DEFAULT_HOME_EXPERIENCE_ID: HomeExperienceId = 'classic'

export const HOME_EXPERIENCES: HomeExperienceDefinition[] = [
  {
    id: 'classic',
    component: ClassicHomeExperience,
  },
]
