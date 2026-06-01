import { ClassicHomeExperience } from './experiences/classic/ClassicHomeExperience'
import { PremiumLocalHomeExperience } from './experiences/premiumLocal/PremiumLocalHomeExperience'
import type { HomeExperienceDefinition, HomeExperienceId } from './homeExperienceTypes'

export const DEFAULT_HOME_EXPERIENCE_ID: HomeExperienceId = 'premiumLocal'

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
