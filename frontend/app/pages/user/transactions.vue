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
      <UTable :columns="columns" :rows="transactions">
        <template #amount-data="{ row }">
          <span :class="row.amount >= 0 ? 'text-green-600' : 'text-red-600'">
            {{ formatCurrency(row.amount) }}
          </span>
        </template>
        <template #created_at-data="{ row }">
          {{ formatDate(row.created_at) }}
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
import { useTransactionService } from '~/services'

definePageMeta({
  layout: 'dashboard',
  middleware: 'auth'
})

const transactionService = useTransactionService()
const toast = useToast()

const transactions = ref<TransactionResponse[]>([])
const showCreateTransaction = ref(false)

const columns = [
  { key: 'id', label: 'ID' },
  { key: 'amount', label: 'Amount' },
  { key: 'transaction_type_id', label: 'Type' },
  { key: 'created_at', label: 'Date' }
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
