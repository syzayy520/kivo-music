import type { LocaleCode } from './localeTypes'

export const defaultLocale: LocaleCode = 'en-US'

export const supportedLocales: LocaleCode[] = [
  'zh-CN',
  'zh-TW',
  'en-US',
  'ja-JP',
  'ko-KR',
  'de-DE',
  'fr-FR',
  'es-ES',
  'pt-BR',
]

const localeAliases: Record<string, LocaleCode> = {
  zh: 'zh-CN',
  'zh-Hans': 'zh-CN',
  'zh-Hans-CN': 'zh-CN',
  'zh-SG': 'zh-CN',
  'zh-Hant': 'zh-TW',
  'zh-Hant-TW': 'zh-TW',
  'zh-HK': 'zh-TW',
  'zh-MO': 'zh-TW',
  en: 'en-US',
  ja: 'ja-JP',
  ko: 'ko-KR',
  de: 'de-DE',
  fr: 'fr-FR',
  es: 'es-ES',
  pt: 'pt-BR',
  'pt-PT': 'pt-BR',
}

export function normalizeLocale(locale?: string): LocaleCode {
  if (!locale) {
    return defaultLocale
  }

  if (supportedLocales.includes(locale as LocaleCode)) {
    return locale as LocaleCode
  }

  return localeAliases[locale] ?? localeAliases[locale.split('-')[0]] ?? defaultLocale
}

export function detectLocale(): LocaleCode {
  if (typeof navigator === 'undefined') {
    return defaultLocale
  }

  return normalizeLocale(navigator.languages?.[0] ?? navigator.language)
}
