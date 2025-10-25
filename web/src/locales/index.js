import { createI18n } from 'vue-i18n';
import zhCN from './zh-CN';
import zhTW from './zh-TW';
import zhHK from './zh-HK';
import enUS from './en-US';
import enGB from './en-GB';

const messages = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
  'zh-HK': zhHK,
  'en-US': enUS,
  'en-GB': enGB,
};

function resolveLanguage() {
  const alias = { zh: 'zh-CN', en: 'en-US' };
  const navLangs = navigator.languages || [navigator.language];
  for (const lang of navLangs) {
    if (messages[lang]) {
      return lang;
    }

    if (alias[lang]) {
      return alias[lang];
    }

    const base = lang.split('-')[0];
    if (alias[base]) {
      return alias[base];
    }
  }

  // fallback
  return 'en-US';
}

const i18n = createI18n({
  legacy: false,
  locale: resolveLanguage(),
  fallbackLocale: 'en-US',
  messages,
});

export default i18n;
