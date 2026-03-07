<template>
  <div 
    class="flex items-center justify-between p-4 bg-white dark:bg-gray-900 rounded-2xl border border-gray-100 dark:border-gray-800 cursor-pointer card-hover"
    @click="$emit('click')"
  >
    <div class="flex items-center gap-3">
      <div class="w-11 h-11 rounded-xl flex items-center justify-center shadow-sm" :class="categoryColor">
        <UIcon name="i-lucide-archive" class="w-5 h-5 text-white" />
      </div>
      <div>
        <h4 class="font-semibold text-gray-900 dark:text-white">{{ basket.name }}</h4>
        <p class="text-sm text-gray-500 dark:text-gray-400">{{ basket.description || 'No description' }}</p>
      </div>
    </div>
    <div class="text-right">
      <p class="font-bold text-gray-900 dark:text-white">{{ formatCurrency(basket.balance) }}</p>
      <UBadge size="xs" :color="statusColor" variant="subtle">{{ basket.status }}</UBadge>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { BasketResponse } from '~/dtos'

interface Props {
  basket: BasketResponse
}

const props = defineProps<Props>()

defineEmits<{
  click: []
}>()

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(amount)
}

const categoryColors = [
  'bg-blue-500',
  'bg-green-500',
  'bg-purple-500',
  'bg-pink-500',
  'bg-orange-500',
  'bg-teal-500'
]

const categoryColor = computed(() => {
  const index = props.basket.basket_category_id % categoryColors.length
  return categoryColors[index]
})

const statusColor = computed(() => {
  const colors: Record<string, 'success' | 'warning' | 'error'> = {
    active: 'success',
    archived: 'warning',
    disabled: 'error'
  }
  return colors[props.basket.status] || 'success'
})
</script>
