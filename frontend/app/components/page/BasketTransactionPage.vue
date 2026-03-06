<template>
  <UDashboardPanel id="basket-transactions">
    <template #header>
      <UDashboardNavbar :title="$t('transactions.title')" :ui="{ right: 'gap-3' }">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>
    <template #body>
      <PageHeader
        :title="$t('transactions.title')"
        :description="$t('transactions.description')"
      />
      <UCard>
        <DataTable
          :columns="columns"
          :data="transactions"
          empty-icon="i-heroicons-document-text"
          :empty-message="$t('transactions.noTransactions')"
        >
          <template #amount-data="{ row }">
            <span :class="row.original.amount >= 0 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'">
              {{ formatCurrency(row.original.amount) }}
            </span>
          </template>
          <template #created_at-data="{ row }">
            {{ formatDate(row.original.created_at) }}
          </template>
        </DataTable>
      </UCard>
    </template>
  </UDashboardPanel>
</template>

<script setup lang="ts">
import type { TransactionResponse } from "~/dtos";
import type { TableColumn } from "@nuxt/ui";
import { useTransactions } from "#imports";
import { useRoute } from 'vue-router';

const { t } = useI18n();
const transactionService = useTransactions();
const toast = useToast();
const route = useRoute();

const transactions = ref<TransactionResponse[]>([]);
const columns = computed<TableColumn<TransactionResponse>[]>(() => [
  { accessorKey: "id", header: t("transactions.id") },
  { accessorKey: "amount", header: t("common.amount") },
  { accessorKey: "transaction_type_id", header: t("common.type") },
  { accessorKey: "created_at", header: t("common.date") },
]);

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
  }).format(amount);
};

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
};

async function loadTransactions() {
  try {
    const basketId = Number(route.params.basketId);
    const response = await transactionService.fetchBasketTransactions(basketId, { limit: 50, page: 1 });
    transactions.value = response.data;
  } catch (error) {
    toast.add({ title: t("transactions.loadFailed"), color: "error" });
  }
}

onMounted(() => {
  loadTransactions();
});
</script>
