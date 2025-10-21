import { createRouter, createWebHashHistory } from "vue-router";

import About from "../views/About.vue";
import Home from "../views/Home.vue";
import Terminal from "../views/Terminal.vue";

const routes = [
    { path: '/', name: 'Home', component: Home },
  { path: '/about', name: 'About', component: About },
  { path: '/terminal', name: 'Terminal', component: Terminal },
]

const router = createRouter({
  history: createWebHashHistory(), // usa la history API (buono per SPA)
  routes,
})

export default router