<template>
  <UDashboardPanel id="user-basket">
    <template #header>
      <UDashboardNavbar title="Home" :ui="{ right: 'gap-3' }">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>
    <template #body>
      <PageHeader
        :title="$t('baskets.title')"
        :description="$t('baskets.description')"
        :action-label="$t('baskets.createBasket')"
        action-icon="i-lucide-plus"
        @action="showCreateBasket = true"
      />

      <!-- Main Wallet Card -->
      <div
        v-if="mainBasket"
        class="mb-8 bg-linear-to-r from-amber-400 via-amber-500 to-orange-500 rounded-2xl p-6 shadow-xl shadow-amber-400/20 relative overflow-hidden"
      >
        <div class="absolute top-0 right-0 w-40 h-40 bg-white/10 rounded-full -translate-y-1/2 translate-x-1/4" />
        <div class="absolute bottom-0 left-0 w-24 h-24 bg-white/10 rounded-full translate-y-1/2 -translate-x-1/4" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-amber-900/70 text-sm font-medium">
              {{ $t("dashboard.mainWallet") }}
            </p>
            <h2 class="text-3xl font-extrabold text-white mt-1">
              {{ formatCurrency(mainBasket.balance) }}
            </h2>
          </div>
          <div
            class="w-16 h-16 bg-white/20 backdrop-blur-sm rounded-2xl flex items-center justify-center"
          >
            <UIcon name="i-lucide-wallet" class="w-8 h-8 text-white" />
          </div>
        </div>
      </div>

      <!-- Branch Baskets Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 stagger-children">
        <div
          v-for="basket in branchBaskets"
          :key="basket.id"
          class="bg-white dark:bg-gray-900 rounded-2xl border border-gray-100 dark:border-gray-800 p-5 card-hover"
        >
          <div class="flex items-start justify-between mb-4">
            <div>
              <h3 class="font-bold text-gray-900 dark:text-white">
                {{ basket.name }}
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                {{ basket.description || $t("common.noDescription") }}
              </p>
            </div>
            <UBadge :color="getStatusColor(basket.status)" variant="subtle" size="xs">
              {{ basket.status }}
            </UBadge>
          </div>
          <p class="text-2xl font-extrabold text-gray-900 dark:text-white mb-4">
            {{ formatCurrency(basket.balance) }}
          </p>
          <UButton
            size="xs"
            variant="soft"
            color="primary"
            @click="navigateTo(`/user/baskets/${basket.id}/transactions`)"
          >
            <UIcon name="i-lucide-arrow-right" class="w-3.5 h-3.5 mr-1" />
            {{ $t("baskets.viewTransactions") }}
          </UButton>
        </div>
      </div>

      <EmptyState
        v-if="branchBaskets.length === 0"
        icon="i-lucide-archive"
        :message="$t('baskets.noBaskets')"
      />

      <!-- Create Basket Modal -->
      <UModal v-model:open="showCreateBasket">
        <template #content>
          <CreateBasketForm
            @success="handleBasketCreated"
            @cancel="showCreateBasket = false"
          />
        </template>
      </UModal>
    </template>
  </UDashboardPanel>
</template>

<script setup lang="ts">
import type { BasketResponse } from "~/dtos";
import { useBaskets } from "#imports";

const { t } = useI18n();
const basketService = useBaskets();
const toast = useToast();

const mainBasket = ref<BasketResponse | null>(null);
const branchBaskets = ref<BasketResponse[]>([]);
const showCreateBasket = ref(false);

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
  }).format(amount);
};

const getStatusColor = (status: string) => {
  const colors: Record<string, "success" | "warning" | "error" | "info"> = {
    active: "success",
    archived: "warning",
    disabled: "error",
  };
  return colors[status] || "info";
};

async function loadBaskets() {
  try {
    const response = await basketService.fetchBaskets();
    mainBasket.value =
      response.data.find((b) => b.basket_type === "main") || null;
    branchBaskets.value = response.data.filter((b) => b.basket_type !== "main");
  } catch (error) {
    toast.add({ title: t("baskets.loadFailed"), color: "error" });
  }
}

function handleBasketCreated() {
  showCreateBasket.value = false;
  loadBaskets();
  toast.add({ title: t("baskets.basketCreated"), color: "success" });
}

onMounted(() => {
  loadBaskets();
});
</script>
