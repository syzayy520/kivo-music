import {
  ACTIVE_HOME_EXPERIENCE_ID,
  resolveHomeExperience,
} from './homeExperienceRegistry'
import type { HomeExperienceId } from './homeExperienceTypes'

type HomeExperienceHostProps = {
  experienceId?: HomeExperienceId
}

export function HomeExperienceHost({
  experienceId = ACTIVE_HOME_EXPERIENCE_ID,
}: HomeExperienceHostProps) {
  const selectedExperience = resolveHomeExperience(experienceId)

  if (!selectedExperience) return <></>

  const Experience = selectedExperience.component
  return <Experience />
}
