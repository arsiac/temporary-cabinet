import { createApp } from 'vue';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import dayjsPlugin from '@/plugins/dayjs';
import router from './router';
import i18n from './locales';
import App from './App.vue';

const app = createApp(App);

const elLocaleMap = {
  'zh-CN': () => import('element-plus/es/locale/lang/zh-cn'),
  'zh-TW': () => import('element-plus/es/locale/lang/zh-tw'),
  'zh-HK': () => import('element-plus/es/locale/lang/zh-hk'),
  'en-US': () => import('element-plus/es/locale/lang/en'),
  'en-GB': () => import('element-plus/es/locale/lang/en'),
};
const loadElLocale = async () => {
  const lang = i18n.global.locale.value;
  const module = await (elLocaleMap[lang] || elLocaleMap['en-US'])();
  app.config.globalProperties.$ELEMENT = { locale: module.default };
};

loadElLocale();

app.use(ElementPlus);
app.use(i18n);
app.use(dayjsPlugin);
app.use(router);
app.mount('#app');
