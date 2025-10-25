import { createRouter, createWebHistory } from 'vue-router';
import Home from './views/Home.vue';
import Pick from './views/Pick.vue';
import i18n from './locales';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'home', component: Home, meta: { title: 'title:temporary-cabinet' } },
    {
      path: '/pick',
      name: 'pick',
      component: Pick,
      meta: { title: 'title:temporary-cabinet-pickup' },
    },
  ],
});

router.afterEach((to) => {
  if (to.meta.title) {
    document.title = i18n.global.t(to.meta.title);
  } else {
    document.title = i18n.global.t('temporary-cabinet');
  }
});

export default router;
