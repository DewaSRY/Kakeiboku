<template>
  <UDashboardPanel id="user-dashboard">
    <template #header>
      <UDashboardNavbar title="Home" :ui="{ right: 'gap-3' }">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>
    <template #body>
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          {{ $t("dashboard.title") }}
        </h1>
        <p class="text-gray-600 dark:text-gray-400 mt-1">
          {{ $t("dashboard.subtitle") }}
        </p>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <DashboardStatCard
          :title="$t('dashboard.totalBalance')"
          :value="formatCurrency(totalBalance)"
          icon="i-heroicons-banknotes"
          color="amber"
        />

        <DashboardStatCard
          :title="$t('dashboard.mainWallet')"
          :value="formatCurrency(mainBasket?.balance || 0)"
          icon="i-heroicons-wallet"
          color="green"
        />
        
        <DashboardStatCard
          :title="$t('dashboard.branchBasketsCount')"
          :value="branchBaskets.length.toString()"
          icon="i-heroicons-archive-box"
          color="blue"
        />
      </div>

      <!-- Main Content Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Branch Baskets -->
        <PageContentCard :title="$t('dashboard.branchBaskets')">
          <template #header-action>
            <UButton
              color="primary"
              size="sm"
              icon="i-heroicons-plus"
              @click="showCreateBasket = true"
            >
              {{ $t("dashboard.newBasket") }}
            </UButton>
          </template>

          <PageEmptyState
            v-if="branchBaskets.length === 0"
            icon="i-heroicons-archive-box"
            :message="$t('dashboard.noBranchBaskets')"
            padding="sm"
          />

          <div v-else class="space-y-4">
            <BasketItem
              v-for="basket in branchBaskets"
              :key="basket.id"
              :basket="basket"
              @click="navigateTo(`/user/baskets/${basket.id}`)"
            />
          </div>
        </PageContentCard>

        <!-- Recent Transactions -->
        <PageContentCard :title="$t('dashboard.recentTransactions')">
          <template #header-action>
            <UButton
              color="neutral"
              variant="ghost"
              size="sm"
              @click="navigateTo('/user/transactions')"
            >
              {{ $t("common.viewAll") }}
            </UButton>
          </template>

          <PageEmptyState
            v-if="recentTransactions.length === 0"
            icon="i-heroicons-arrow-path"
            :message="$t('dashboard.noTransactions')"
            padding="sm"
          />

          <div v-else class="space-y-4">
            <TransactionItem
              v-for="transaction in recentTransactions"
              :key="transaction.id"
              :transaction="transaction"
            />
          </div>
        </PageContentCard>
      </div>

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
import type { BasketResponse, TransactionResponse } from "~/dtos";
import { useBaskets, useTransactions } from "#imports";

definePageMeta({
  layout: "dashboard",
});

const { t } = useI18n();
const basketService = useBaskets();
const transactionService = useTransactions();
const toast = useToast();

const totalBalance = ref(0);
const mainBasket = ref<BasketResponse | null>(null);
const branchBaskets = ref<BasketResponse[]>([]);
const recentTransactions = ref<TransactionResponse[]>([]);
const showCreateBasket = ref(false);

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
  }).format(amount);
};

async function loadDashboardData() {
  try {
    const [basketsResponse, transactions] = await Promise.all([
      basketService.fetchBaskets(),
      transactionService.getRecentTransactions(5),
    ]);

    const allBaskets = basketsResponse.data;
    mainBasket.value = allBaskets.find((b) => b.basket_type === "main") || null;
    branchBaskets.value = allBaskets.filter((b) => b.basket_type !== "main");
    totalBalance.value = allBaskets.reduce((acc, b) => acc + b.balance, 0);
    recentTransactions.value = transactions;
  } catch (error) {
    toast.add({ title: t("common.error"), color: "error" });
  }
}

function handleBasketCreated() {
  showCreateBasket.value = false;
  loadDashboardData();
  toast.add({ title: t("baskets.basketCreated"), color: "success" });
}

onMounted(() => {
  loadDashboardData();
});
</script>
