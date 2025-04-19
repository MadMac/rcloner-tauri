import { createMemoryHistory, createRouter } from "vue-router";
import HomeView from "./views/Home.vue";
import CopyStart from "./views/CopyStart.vue";
import CopyDryRun from "./views/CopyDryRun.vue";
import CopyStartRun from "./views/CopyStartRun.vue";

const routes = [
  { path: "/", component: HomeView },
  { path: "/copy/start", component: CopyStart },
  { path: "/copy/dry-run", component: CopyDryRun },
  { path: "/copy/start-run", component: CopyStartRun },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
