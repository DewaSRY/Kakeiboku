<template>
  <div>
    <div class="flex items-center justify-between mb-8">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Transactions</h1>
        <p class="text-gray-600 mt-1">View and manage all your transactions.</p>
      </div>
      <UButton 
        color="primary" 
        icon="i-heroicons-plus"
        @click="showCreateTransaction = true"
      >
        New Transaction
      </UButton>
    </div>

    <UCard>
      <UTable :columns="columns" :data="transactions">
        <template #amount-data="{ row }">
          <span :class="row.original.amount >= 0 ? 'text-green-600' : 'text-red-600'">
            {{ formatCurrency(row.original.amount) }}
          </span>
        </template>
        <template #created_at-data="{ row }">
          {{ formatDate(row.original.created_at) }}
        </template>
      </UTable>

      <div v-if="transactions.length === 0" class="text-center py-12">
        <UIcon name="i-heroicons-document-text" class="w-12 h-12 text-gray-300 mx-auto mb-4" />
        <p class="text-gray-500">No transactions found.</p>
      </div>
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
  </div>
</template>

<script setup lang="ts">
import type { TransactionResponse } from '~/dtos'
import type { TableColumn } from '@nuxt/ui'
import { useTransactionService } from '~/services'

definePageMeta({
  layout: 'dashboard',
})

const transactionService = useTransactionService()
const toast = useToast()

const transactions = ref<TransactionResponse[]>([])
const showCreateTransaction = ref(false)

const columns: TableColumn<TransactionResponse>[] = [
  { accessorKey: 'id', header: 'ID' },
  { accessorKey: 'amount', header: 'Amount' },
  { accessorKey: 'transaction_type_id', header: 'Type' },
  { accessorKey: 'created_at', header: 'Date' }
]

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(amount)
}

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

async function loadTransactions() {
  try {
    const response = await transactionService.getAllUserTransactions({ limit: 50, page: 1 })
    transactions.value = response.data
  } catch (error) {
    toast.add({ title: 'Failed to load transactions', color: 'error' })
  }
}

function handleTransactionCreated() {
  showCreateTransaction.value = false
  loadTransactions()
  toast.add({ title: 'Transaction created successfully!', color: 'success' })
}

onMounted(() => {
  loadTransactions()
})
</script>
