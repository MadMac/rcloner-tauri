import { createMemoryHistory, createRouter } from 'vue-router'
import HomeView from './views/Home.vue'
import CopyStart from './views/CopyStart.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/copy/start', component: CopyStart },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router