export type HomeExperienceShellMode = 'classic' | 'premiumLocal'

type ShellModeSource = {
  id: string
  shellMode: HomeExperienceShellMode
}

export const DEFAULT_HOME_EXPERIENCE_SHELL_MODE: HomeExperienceShellMode = 'classic'

export function resolveHomeExperienceShellMode(
  experiences: ShellModeSource[],
  id: string,
): HomeExperienceShellMode {
  return (
    experiences.find((experience) => experience.id === id)?.shellMode ??
    DEFAULT_HOME_EXPERIENCE_SHELL_MODE
  )
}
