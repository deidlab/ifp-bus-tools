<template>
  <header
    class="z-40"
    :class="{ dark: store.semidark && store.menu === 'horizontal' }"
  >
    <div class="shadow-sm">
      <div
        class="relative bg-white flex w-full items-center px-5 py-2.5 dark:bg-[#0e1726]"
      >
        <div
          class="horizontal-logo flex lg:hidden justify-between items-center ltr:mr-2 rtl:ml-2"
        >
          <router-link to="/" class="main-logo flex items-center shrink-0">
            <img
              class="w-8 ltr:-ml-1 rtl:-mr-1 inline"
              src="/assets/images/logo.svg"
              alt=""
            />
            <span
              class="text-2xl ltr:ml-1.5 rtl:mr-1.5 font-semibold align-middle hidden md:inline dark:text-white-light transition-all duration-300"
              >IFP Bus Tools</span
            >
          </router-link>

          <a
            href="javascript:;"
            class="collapse-icon flex-none dark:text-[#d0d2d6] hover:text-primary dark:hover:text-primary flex lg:hidden ltr:ml-2 rtl:mr-2 p-2 rounded-full bg-white-light/40 dark:bg-dark/40 hover:bg-white-light/90 dark:hover:bg-dark/60"
            @click="store.toggleSidebar()"
          >
            <icon-menu class="w-5 h-5" />
          </a>
        </div>

        <div
          class="sm:flex-1 ltr:sm:ml-0 ltr:ml-auto sm:rtl:mr-0 rtl:mr-auto flex items-center space-x-1.5 lg:space-x-2 rtl:space-x-reverse dark:text-[#d0d2d6]"
        >
          <div class="sm:ltr:mr-auto sm:rtl:ml-auto"></div>
          <div>
            <a
              href="javascript:;"
              v-show="store.theme === 'light'"
              class="flex items-center p-2 rounded-full bg-white-light/40 dark:bg-dark/40 hover:text-primary hover:bg-white-light/90 dark:hover:bg-dark/60"
              @click="store.toggleTheme('dark')"
            >
              <icon-sun />
            </a>
            <a
              href="javascript:;"
              v-show="store.theme === 'dark'"
              class="flex items-center p-2 rounded-full bg-white-light/40 dark:bg-dark/40 hover:text-primary hover:bg-white-light/90 dark:hover:bg-dark/60"
              @click="store.toggleTheme('system')"
            >
              <icon-moon />
            </a>
            <a
              href="javascript:;"
              v-show="store.theme === 'system'"
              class="flex items-center p-2 rounded-full bg-white-light/40 dark:bg-dark/40 hover:text-primary hover:bg-white-light/90 dark:hover:bg-dark/60"
              @click="store.toggleTheme('light')"
            >
              <icon-laptop />
            </a>
          </div>

          <div class="dropdown shrink-0">
            <Popper
              :placement="
                store.rtlClass === 'rtl' ? 'bottom-end' : 'bottom-start'
              "
              offsetDistance="8"
            >
              <button
                type="button"
                class="block p-2 rounded-full bg-white-light/40 dark:bg-dark/40 hover:text-primary hover:bg-white-light/90 dark:hover:bg-dark/60"
              >
                <img
                  :src="currentFlag"
                  alt="flag"
                  class="w-5 h-5 object-cover rounded-full"
                />
              </button>
              <template #content="{ close }">
                <ul
                  class="!px-2 text-dark dark:text-white-dark grid grid-cols-2 gap-2 font-semibold dark:text-white-light/90 w-[280px]"
                >
                  <template v-for="item in store.languageList" :key="item.code">
                    <li>
                      <button
                        type="button"
                        class="w-full hover:text-primary"
                        :class="{
                          'bg-primary/10 text-primary':
                            i18n.locale === item.code,
                        }"
                        @click="changeLanguage(item), close()"
                      >
                        <img
                          class="w-5 h-5 object-cover rounded-full"
                          :src="`/assets/images/flags/${item.code.toUpperCase()}.svg`"
                          alt=""
                        />
                        <span class="ltr:ml-3 rtl:mr-3">{{ item.name }}</span>
                      </button>
                    </li>
                  </template>
                </ul>
              </template>
            </Popper>
          </div>

          <div class="dropdown shrink-0">
            <Popper
              :placement="
                store.rtlClass === 'rtl' ? 'bottom-end' : 'bottom-start'
              "
              offsetDistance="8"
              class="!block"
            >
              <button
                type="button"
                class="relative block p-2 rounded-full bg-white-light/40 dark:bg-dark/40 hover:text-primary hover:bg-white-light/90 dark:hover:bg-dark/60"
              >
                <icon-bell-bing />
              </button>
              <template #content="{ close }">
                <ul
                  class="text-dark dark:text-white-dark !py-0 w-[230px] font-semibold dark:text-white-light/90"
                >
                  <li>
                    <router-link
                      to="/users/profile"
                      class="dark:hover:text-white"
                      @click="close()"
                    >
                      <icon-user
                        class="w-4.5 h-4.5 ltr:mr-2 rtl:ml-2 shrink-0"
                      />

                      Profile
                    </router-link>
                  </li>
                  <li>
                    <router-link
                      to="/apps/mailbox"
                      class="dark:hover:text-white"
                      @click="close()"
                    >
                      <icon-mail
                        class="w-4.5 h-4.5 ltr:mr-2 rtl:ml-2 shrink-0"
                      />

                      Inbox
                    </router-link>
                  </li>
                  <li>
                    <router-link
                      to="/auth/boxed-lockscreen"
                      class="dark:hover:text-white"
                      @click="close()"
                    >
                      <icon-lock-dots
                        class="w-4.5 h-4.5 ltr:mr-2 rtl:ml-2 shrink-0"
                      />

                      Lock Screen
                    </router-link>
                  </li>
                  <li
                    class="border-t border-white-light dark:border-white-light/10"
                  >
                    <router-link
                      to="/auth/boxed-signin"
                      class="text-danger !py-3"
                      @click="close_app()"
                    >
                      <icon-logout
                        class="w-4.5 h-4.5 ltr:mr-2 rtl:ml-2 rotate-90 shrink-0"
                      />

                      Sign Out
                    </router-link>
                  </li>
                </ul>
              </template>
            </Popper>
          </div>
        </div>
      </div>

      <!-- horizontal menu -->
      <ul
        class="horizontal-menu hidden py-1.5 font-semibold px-6 lg:space-x-1.5 xl:space-x-8 rtl:space-x-reverse bg-white border-t border-[#ebedf2] dark:border-[#191e3a] dark:bg-[#0e1726] text-black dark:text-white-dark"
      >
        <li class="menu nav-item relative">
          <a href="javascript:;" class="nav-link">
            <div class="flex items-center">
              <icon-menu-dashboard class="shrink-0" />

              <span class="px-2">{{ $t("utilities.name") }}</span>
            </div>
            <div class="right_arrow">
              <icon-caret-down />
            </div>
          </a>
          <ul class="sub-menu">
            <li>
              <router-link to="/utilities/serial_terminal">{{
                $t("utilities.serial_terminal")
              }}</router-link>
            </li>
            <li>
              <router-link to="/utilities/tcp_client">{{
                $t("utilities.tcp_client")
              }}</router-link>
            </li>
            <li>
              <router-link to="/utilities/tcp_server">{{
                $t("utilities.tcp_server")
              }}</router-link>
            </li>
            <li>
              <router-link to="/utilities/udp_terminal">{{
                $t("utilities.udp_terminal")
              }}</router-link>
            </li>
          </ul>
        </li>
        <li class="menu nav-item relative">
          <a href="javascript:;" class="nav-link">
            <div class="flex items-center">
              <icon-menu-apps class="shrink-0" />

              <span class="px-2">{{ $t("apps") }}</span>
            </div>
            <div class="right_arrow">
              <icon-caret-down />
            </div>
          </a>
          <ul class="sub-menu">
            <li>
              <router-link to="/apps/chat">{{ $t("chat") }}</router-link>
            </li>
            <li>
              <router-link to="/apps/mailbox">{{ $t("mailbox") }}</router-link>
            </li>
            <li>
              <router-link to="/apps/todolist">{{
                $t("todo_list")
              }}</router-link>
            </li>
            <li>
              <router-link to="/apps/notes">{{ $t("notes") }}</router-link>
            </li>
            <li>
              <router-link to="/apps/scrumboard">{{
                $t("scrumboard")
              }}</router-link>
            </li>
            <li>
              <router-link to="/apps/contacts">{{
                $t("contacts")
              }}</router-link>
            </li>
            <li class="relative">
              <a href="javascript:;"
                >{{ $t("invoice") }}
                <div class="ltr:ml-auto rtl:mr-auto rtl:rotate-90 -rotate-90">
                  <icon-caret-down />
                </div>
              </a>
              <ul
                class="rounded absolute top-0 ltr:left-[95%] rtl:right-[95%] min-w-[180px] bg-white z-[10] text-dark dark:text-white-dark dark:bg-[#1b2e4b] shadow p-0 py-2 hidden"
              >
                <li>
                  <router-link to="/apps/invoice/list">{{
                    $t("list")
                  }}</router-link>
                </li>
                <li>
                  <router-link to="/apps/invoice/preview">{{
                    $t("preview")
                  }}</router-link>
                </li>
                <li>
                  <router-link to="/apps/invoice/add">{{
                    $t("add")
                  }}</router-link>
                </li>
                <li>
                  <router-link to="/apps/invoice/edit">{{
                    $t("edit")
                  }}</router-link>
                </li>
              </ul>
            </li>
            <li>
              <router-link to="/apps/calendar">{{
                $t("calendar")
              }}</router-link>
            </li>
          </ul>
        </li>
        <li class="menu nav-item relative">
          <a href="javascript:;" class="nav-link">
            <div class="flex items-center">
              <icon-menu-components class="shrink-0" />
              <span class="px-2">{{ $t("components") }}</span>
            </div>
            <div class="right_arrow">
              <icon-caret-down />
            </div>
          </a>
          <ul class="sub-menu">
            <li>
              <router-link to="/components/tabs">{{ $t("tabs") }}</router-link>
            </li>
            <li>
              <router-link to="/components/accordions">{{
                $t("accordions")
              }}</router-link>
            </li>
            <li>
              <router-link to="/components/modals">{{
                $t("modals")
              }}</router-link>
            </li>
            <li>
              <router-link to="/components/cards">{{
                $t("cards")
              }}</router-link>
            </li>
            <li>
              <router-link to="/components/carousel">{{
                $t("carousel")
              }}</router-link>
            </li>
            <li>
              <router-link to="/components/countdown">{{
                $t("countdown")
              }}</router-link>
            </li>
            <li>
              <router-link to="/components/counter">{{
                $t("counter")
              }}</router-link>
            </li>
            <li>
              <router-link to="/components/sweetalert">{{
                $t("sweet_alerts")
              }}</router-link>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { onMounted, computed, reactive, watch } from "vue";
import { useI18n } from "vue-i18n";
import { exit } from "@tauri-apps/plugin-process";
import appSetting from "@/app-setting";

import { useRoute } from "vue-router";
import { useAppStore } from "@/stores/index";

import IconMenu from "@/components/icon/icon-menu.vue";
import IconSun from "@/components/icon/icon-sun.vue";
import IconMoon from "@/components/icon/icon-moon.vue";
import IconLaptop from "@/components/icon/icon-laptop.vue";
import IconBellBing from "@/components/icon/icon-settings.vue";
import IconUser from "@/components/icon/icon-user.vue";
import IconMail from "@/components/icon/icon-mail.vue";
import IconLockDots from "@/components/icon/icon-lock-dots.vue";
import IconLogout from "@/components/icon/icon-logout.vue";
import IconMenuDashboard from "@/components/icon/menu/icon-menu-dashboard.vue";
import IconCaretDown from "@/components/icon/icon-caret-down.vue";
import IconMenuApps from "@/components/icon/menu/icon-menu-apps.vue";
import IconMenuComponents from "@/components/icon/menu/icon-menu-components.vue";

const store = useAppStore();
const route = useRoute();

// multi language
const i18n = reactive(useI18n());
const changeLanguage = (item: any) => {
  i18n.locale = item.code;
  appSetting.toggleLanguage(item);
};
const currentFlag = computed(() => {
  return `/assets/images/flags/${i18n.locale.toUpperCase()}.svg`;
});

onMounted(() => {
  setActiveDropdown();
});

watch(route, (to, from) => {
  setActiveDropdown();
});

const setActiveDropdown = () => {
  const selector = document.querySelector(
    'ul.horizontal-menu a[href="' + window.location.pathname + '"]'
  );
  if (selector) {
    selector.classList.add("active");
    const all: any = document.querySelectorAll(
      "ul.horizontal-menu .nav-link.active"
    );
    for (let i = 0; i < all.length; i++) {
      all[0]?.classList.remove("active");
    }
    const ul: any = selector.closest("ul.sub-menu");
    if (ul) {
      let ele: any = ul.closest("li.menu").querySelectorAll(".nav-link");
      if (ele) {
        ele = ele[0];
        setTimeout(() => {
          ele?.classList.add("active");
        });
      }
    }
  }
};

async function close_app() {
  await exit(0);
}
</script>
