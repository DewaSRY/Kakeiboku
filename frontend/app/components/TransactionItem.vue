<template>
  <div class="flex items-center justify-between py-3 border-b border-gray-100 last:border-0">
    <div class="flex items-center space-x-3">
      <div 
        class="w-10 h-10 rounded-full flex items-center justify-center"
        :class="transaction.amount >= 0 ? 'bg-green-100' : 'bg-red-100'"
      >
        <UIcon 
          :name="transaction.amount >= 0 ? 'i-heroicons-arrow-down-left' : 'i-heroicons-arrow-up-right'" 
          class="w-5 h-5"
          :class="transaction.amount >= 0 ? 'text-green-500' : 'text-red-500'"
        />
      </div>
      <div>
        <p class="font-medium text-gray-900">Transaction #{{ transaction.id }}</p>
        <p class="text-sm text-gray-500">{{ formatDate(transaction.created_at) }}</p>
      </div>
    </div>
    <div class="text-right">
      <p 
        class="font-semibold"
        :class="transaction.amount >= 0 ? 'text-green-600' : 'text-red-600'"
      >
        {{ transaction.amount >= 0 ? '+' : '' }}{{ formatCurrency(transaction.amount) }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TransactionResponse } from '~/dtos'

interface Props {
  transaction: TransactionResponse
}

defineProps<Props>()

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(Math.abs(amount))
}

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>
