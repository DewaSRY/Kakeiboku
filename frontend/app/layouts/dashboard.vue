<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";
const { t } = useI18n();
const open = ref(false);

const links = [
  {
    label: t("nav.dashboard"),
    icon: "i-lucide-home",
    to: "/user/dashboard",
    onSelect: () => {
      open.value = false;
    },
  },
  {
    label: t("nav.baskets"),
    icon: "i-lucide-archive",
    to: "/user/baskets",
    onSelect: () => {
      open.value = false;
    },
  },
  // Removed direct transactions link. Transactions are now accessed via baskets.
] satisfies NavigationMenuItem[];
</script>

<template>
  <UDashboardGroup unit="rem">
    <UDashboardSidebar
      id="default"
      v-model:open="open"
      collapsible
      resizable
      class="bg-elevated/25"
      :ui="{ footer: 'lg:border-t lg:border-default' }"
    >
      <template #header="{ collapsed }">
        <NuxtLink to="/user/dashboard" class="flex items-center gap-2.5 p-2 group">
          <div class="w-9 h-9 bg-linear-to-br from-amber-400 to-amber-500 rounded-xl flex items-center justify-center shadow-md shadow-amber-400/20 shrink-0">
            <span class="text-white font-bold text-lg">K</span>
          </div>
          <span v-if="!collapsed" class="text-lg font-bold text-gray-900 dark:text-white truncate">Kakeiboku</span>
        </NuxtLink>
      </template>

      <template #default="{ collapsed }">
        <UDashboardSearchButton
          :collapsed="collapsed"
          class="bg-transparent ring-default"
        />

        <UNavigationMenu
          :collapsed="collapsed"
          :items="links"
          orientation="vertical"
          tooltip
          popover
        />
      </template>

      <template #footer="{ collapsed }">
        <div class="flex items-center" :class="collapsed ? 'justify-center' : 'gap-2'">
          <ColorModeToggle />
          <LanguageSwitcher v-if="!collapsed" />
        </div>
      </template>

    </UDashboardSidebar>

    <!-- <UDashboardSearch :groups="groups" /> -->

    <slot />

  </UDashboardGroup>
</template>
