import { createRouter, createWebHistory } from 'vue-router';
import Home from './views/Home.vue';
import Pick from './views/Pick.vue';

export default createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/pick', name: 'pick', component: Pick },
  ],
});
