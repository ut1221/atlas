import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "layout",
      component: () => import("@/views/layout/index.vue"),
      redirect: { name: "index" }, //输入路由center会重定向到one页面

      children: [
        {
          path: "index",
          name: "index",
          component: () => import("@/views/index.vue"),
        },
        {
          path: "server",
          name: "exec",
          component: () => import("@/views/server/index.vue"),
        },
        {
          path: "exec",
          name: "exec",
          component: () => import("@/views/exec/index.vue"),
        },
        {
          path: "setting",
          name: "setting",
          component: () => import("@/views/setting/index.vue"),
        },

        {
          path: "demo",
          name: "demo",
          component: () => import("@/views/demo/index.vue"),
        },
      ],
    },
  ],
});

export default router;
