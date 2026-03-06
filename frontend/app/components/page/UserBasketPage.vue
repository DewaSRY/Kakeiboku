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
        action-icon="i-heroicons-plus"
        @action="showCreateBasket = true"
      />

      <UCard
        v-if="mainBasket"
        class="mb-6 bg-linear-to-r from-amber-50 to-amber-100 dark:from-amber-900/30 dark:to-amber-800/30"
      >
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-amber-700 dark:text-amber-400 font-medium">
              {{ $t("dashboard.mainWallet") }}
            </p>
            <h2 class="text-3xl font-bold text-gray-900 dark:text-white mt-1">
              {{ formatCurrency(mainBasket.balance) }}
            </h2>
          </div>
          <div
            class="w-16 h-16 bg-amber-400 rounded-xl flex items-center justify-center"
          >
            <UIcon name="i-heroicons-wallet" class="w-8 h-8 text-white" />
          </div>
        </div>
      </UCard>

      <!-- Branch Baskets Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <UCard
          v-for="basket in branchBaskets"
          :key="basket.id"
          class="cursor-pointer hover:shadow-md transition-shadow"
        >
          <div class="flex items-start justify-between">
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white">
                {{ basket.name }}
              </h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                {{ basket.description || $t("common.noDescription") }}
              </p>
              <UButton
                size="xs"
                class="mt-2"
                @click="navigateTo(`/user/baskets/${basket.id}/transactions`)"
              >
                {{ $t("baskets.viewTransactions") }}
              </UButton>
            </div>
            <UBadge :color="getStatusColor(basket.status)">
              {{ basket.status }}
            </UBadge>
          </div>
          <div class="mt-4">
            <p class="text-2xl font-bold text-gray-900 dark:text-white">
              {{ formatCurrency(basket.balance) }}
            </p>
          </div>
        </UCard>
      </div>

      <EmptyState
        v-if="branchBaskets.length === 0"
        icon="i-heroicons-archive-box"
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
