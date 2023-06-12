import { createRouter, createWebHistory } from 'vue-router';
import Main from "../components/Main.vue";
import Reg from "../components/Registration.vue";
import Auth from "../components/Auth.vue";
import Lib from "../components/Lib.vue";
import Store from "../components/Mag.vue";
import Profile from "../components/Profile.vue";
import Game from "../components/Game.vue";
import Changes from "../components/Changes.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: Main },
    { path: '/signup', component: Reg },
    { path: '/signin', component: Auth },
    { path: '/lib', component: Lib },
    { path: '/Store', component: Store },
    { path: '/Profile', component: Profile },
    { path: '/Profile/Edit', component: Changes },
    { path: '/:game', component: Game }
  ]
})

export default router