<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";
const { t } = useI18n();
const open = ref(false);

const links = [
  {
    label: t("nav.dashboard"),
    icon: "i-heroicons-home",
    to: "/user/dashboard",
    onSelect: () => {
      open.value = false;
    },
  },
  {
    label: t("nav.baskets"),
    icon: "i-heroicons-archive-box",
    to: "/user/baskets",
    onSelect: () => {
      open.value = false;
    },
  },
  {
    label: t("nav.transactions"),
    icon: "i-heroicons-arrow-path",
    to: "/user/transactions",
    onSelect: () => {
      open.value = false;
    },
  },
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
      <!-- <template #header="{ collapsed }">
        <TeamsMenu :collapsed="collapsed" />
      </template> -->

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

    </UDashboardSidebar>

    <!-- <UDashboardSearch :groups="groups" /> -->

    <slot />

  </UDashboardGroup>
</template>
