import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import { useAppStore } from '@/stores/index';
import appSetting from '@/app-setting';

import HomeView from '../views/index.vue';
import SerialTerminal from '../views/utilities/serial_terminal.vue'
import TcpClient from '../views/utilities/tcp_client.vue'
import TcpServer from '../views/utilities/tcp_server.vue'
import UdpTerminal from '../views/utilities/udp_terminal.vue'


const routes: RouteRecordRaw[] = [
    // dashboard
    { path: '/', name: 'homeview', component: HomeView},
    // utilities
    { path: '/utilities/serial_terminal', name: 'serial_terminal', component: SerialTerminal },
    { path: '/utilities/tcp_client', name: 'tcp_client', component: TcpClient },
    { path: '/utilities/tcp_server', name: 'tcp_server', component: TcpServer },
    { path: '/utilities/udp_terminal', name: 'udp_terminal', component: UdpTerminal },
];

const router = createRouter({
    history: createWebHistory(),
    linkExactActiveClass: 'active',
    routes,
    scrollBehavior(to, from, savedPosition) {
        if (savedPosition) {
            return savedPosition;
        } else {
            return { left: 0, top: 0 };
        }
    },
});

router.beforeEach((to, from, next) => {
    const store = useAppStore();

    if (to?.meta?.layout == 'auth') {
        store.setMainLayout('auth');
    } else {
        store.setMainLayout('app');
    }
    next(true);
});
router.afterEach((to, from, next) => {
    appSetting.changeAnimation();
});
export default router;
