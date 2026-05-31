import { defaultLocale, detectLocale, normalizeLocale, supportedLocales } from './detectLocale'
import type { LocaleCode, TranslationDictionary, TranslationKey, TranslationParams } from './localeTypes'
import { deDE } from './locales/de-DE'
import { enUS } from './locales/en-US'
import { esES } from './locales/es-ES'
import { frFR } from './locales/fr-FR'
import { jaJP } from './locales/ja-JP'
import { koKR } from './locales/ko-KR'
import { ptBR } from './locales/pt-BR'
import { zhCN } from './locales/zh-CN'
import { zhTW } from './locales/zh-TW'

export { defaultLocale, detectLocale, normalizeLocale, supportedLocales }
export type { LocaleCode, TranslationDictionary, TranslationKey, TranslationParams }

export const dictionaries: Record<LocaleCode, TranslationDictionary> = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
  'en-US': enUS,
  'ja-JP': jaJP,
  'ko-KR': koKR,
  'de-DE': deDE,
  'fr-FR': frFR,
  'es-ES': esES,
  'pt-BR': ptBR,
}

let activeLocale = detectLocale()

export function getActiveLocale(): LocaleCode {
  return activeLocale
}

export function setActiveLocale(locale: LocaleCode | string): LocaleCode {
  activeLocale = normalizeLocale(locale)
  return activeLocale
}

function replaceToken(message: string, key: string, value: string | number): string {
  return message.split(`{${key}}`).join(String(value))
}

function formatTranslation(template: string, params?: TranslationParams): string {
  if (!params) {
    return template
  }

  return Object.entries(params).reduce(
    (message, [key, value]) => replaceToken(message, key, value),
    template,
  )
}

export function translate(
  key: TranslationKey,
  params?: TranslationParams,
  locale: LocaleCode = activeLocale,
): string {
  const template = dictionaries[locale]?.[key] ?? dictionaries[defaultLocale][key] ?? key
  return formatTranslation(template, params)
}

export const t = translate
