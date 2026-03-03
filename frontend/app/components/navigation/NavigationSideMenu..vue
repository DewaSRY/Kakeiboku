<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const open = ref(false);

const links = [
    {
      label: "Home",
      icon: "i-lucide-house",
      to: "/",
      onSelect: () => {
        open.value = false;
      },
    },
    {
      label: "Inbox",
      icon: "i-lucide-inbox",
      to: "/inbox",
      badge: "4",
      onSelect: () => {
        open.value = false;
      },
    },
    {
      label: "Customers",
      icon: "i-lucide-users",
      to: "/customers",
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

      <!-- <template #footer="{ collapsed }">
        <UserMenu :collapsed="collapsed" />
      </template> -->
    </UDashboardSidebar>

    <!-- <UDashboardSearch :groups="groups" /> -->

    <slot />

    <!-- <NotificationsSlideover /> -->
  </UDashboardGroup>
</template>
