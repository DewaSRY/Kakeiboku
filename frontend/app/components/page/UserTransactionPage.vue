<template>
  <UDashboardPanel id="user-transactions">
    <template #header>
      <UDashboardNavbar title="Transactions" :ui="{ right: 'gap-3' }">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <PageHeader
        :title="$t('transactions.title')"
        :description="$t('transactions.description')"
        :action-label="$t('transactions.createTransaction')"
        action-icon="i-lucide-plus"
        @action="showCreateTransaction = true"
      />

      <UCard>
        <DataTable
          :columns="columns"
          :data="transactions"
          empty-icon="i-lucide-file-text"
          :empty-message="$t('transactions.noTransactions')"
        >
          <template #amount-data="{ row }">
            <span
              :class="
                row.original.amount >= 0
                  ? 'text-green-600 dark:text-green-400'
                  : 'text-red-600 dark:text-red-400'
              "
            >
              {{ formatCurrency(row.original.amount) }}
            </span>
          </template>
          <template #created_at-data="{ row }">
            {{ formatDate(row.original.created_at) }}
          </template>
        </DataTable>
      </UCard>

      <!-- Create Transaction Modal -->
      <UModal v-model:open="showCreateTransaction">
        <template #content>
          <CreateTransactionForm
            @success="handleTransactionCreated"
            @cancel="showCreateTransaction = false"
          />
        </template>
      </UModal>
    </template>
  </UDashboardPanel>
</template>

<script setup lang="ts">
import type { TransactionResponse } from "~/dtos";
import type { TableColumn } from "@nuxt/ui";
import { useTransactions } from "#imports";

definePageMeta({
  layout: "dashboard",
});

const { t } = useI18n();
const transactionService = useTransactions();
const toast = useToast();

const transactions = ref<TransactionResponse[]>([]);
const showCreateTransaction = ref(false);

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
    const response = await transactionService.fetchTransactions({
      limit: 50,
      page: 1,
    });
    transactions.value = response.data;
  } catch (error) {
    toast.add({ title: t("transactions.loadFailed"), color: "error" });
  }
}

function handleTransactionCreated() {
  showCreateTransaction.value = false;
  loadTransactions();
  toast.add({ title: t("transactions.transactionCreated"), color: "success" });
}

onMounted(() => {
  loadTransactions();
});
</script>
