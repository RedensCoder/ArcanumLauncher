import { createRouter, createWebHistory } from 'vue-router';
import Main from "../components/Main.vue";
import Reg from "../components/Registration.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: Main },
    { path: '/reg', component: Reg },
  ]
})

export default router