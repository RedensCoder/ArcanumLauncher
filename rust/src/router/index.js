import { createRouter, createWebHistory } from 'vue-router';
import Main from "../components/Main.vue";
import Reg from "../components/Registration.vue";
import Auth from "../components/Auth.vue";
import Lib from "../components/Lib.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: Main },
    { path: '/signup', component: Reg },
    { path: '/signin', component: Auth },
    { path: '/lib', component: Lib },
  ]
})

export default router