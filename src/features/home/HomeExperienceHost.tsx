import {
  DEFAULT_HOME_EXPERIENCE_ID,
  HOME_EXPERIENCES,
} from './homeExperienceRegistry'
import type { HomeExperienceId } from './homeExperienceTypes'

type HomeExperienceHostProps = {
  experienceId?: HomeExperienceId
}

function findHomeExperience(experienceId: HomeExperienceId) {
  const requestedExperience = HOME_EXPERIENCES.find(
    (experience) => experience.id === experienceId,
  )

  if (requestedExperience) {
    return requestedExperience
  }

  const fallbackExperience = HOME_EXPERIENCES.find(
    (experience) => experience.id === DEFAULT_HOME_EXPERIENCE_ID,
  )

  if (fallbackExperience) {
    return fallbackExperience
  }

  throw new Error('No home experience registered')
}

export function HomeExperienceHost({
  experienceId = DEFAULT_HOME_EXPERIENCE_ID,
}: HomeExperienceHostProps) {
  const selectedExperience = findHomeExperience(experienceId)
  const ExperienceComponent = selectedExperience.component

  return <ExperienceComponent />
}
