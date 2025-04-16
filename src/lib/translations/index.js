import { dev } from '$app/environment';
import i18n from 'sveltekit-i18n';
import en from './en';
import cs from './cs';
import lang from './lang';

/** @type {import('sveltekit-i18n').Config} */
const config = {
  fallbackLocale: 'cs', // Fallback if the key or locale is missing
  log: {
    level: dev ? 'warn' : 'error',
  },
  translations: {
    en: {
      ...en,
      lang,
    },
    cs: {
      ...cs,
      lang,
    },
  },
};

export const defaultLocale = 'cs';
// Export for debugging
// export const rawTranslations = config.translations;

export const { t, locale, locales, loading, loadTranslations } = new i18n(config);