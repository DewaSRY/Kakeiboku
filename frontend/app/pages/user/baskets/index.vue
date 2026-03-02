<template>
  <div>
    <PageHeader
      title="Baskets"
      description="Manage your money baskets."
      action-label="New Basket"
      action-icon="i-heroicons-plus"
      @action="showCreateBasket = true"
    />

    <!-- Main Basket Card -->
    <UCard v-if="mainBasket" class="mb-6 bg-gradient-to-r from-amber-50 to-amber-100 dark:from-amber-900/30 dark:to-amber-800/30">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm text-amber-700 dark:text-amber-400 font-medium">Main Wallet</p>
          <h2 class="text-3xl font-bold text-gray-900 dark:text-white mt-1">
            {{ formatCurrency(mainBasket.balance) }}
          </h2>
        </div>
        <div class="w-16 h-16 bg-amber-400 rounded-xl flex items-center justify-center">
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
        @click="navigateTo(`/user/baskets/${basket.id}`)"
      >
        <div class="flex items-start justify-between">
          <div>
            <h3 class="font-semibold text-gray-900 dark:text-white">{{ basket.name }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">{{ basket.description || 'No description' }}</p>
          </div>
          <UBadge :color="getStatusColor(basket.status)">
            {{ basket.status }}
          </UBadge>
        </div>
        <div class="mt-4">
          <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ formatCurrency(basket.balance) }}</p>
        </div>
      </UCard>
    </div>

    <EmptyState 
      v-if="branchBaskets.length === 0"
      icon="i-heroicons-archive-box"
      message="No branch baskets yet. Create one to start organizing your money!"
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
  </div>
</template>

<script setup lang="ts">
import type { BasketResponse } from '~/dtos'
import { useBasketService } from '~/services'

definePageMeta({
  layout: 'dashboard',
})

const basketService = useBasketService()
const toast = useToast()

const mainBasket = ref<BasketResponse | null>(null)
const branchBaskets = ref<BasketResponse[]>([])
const showCreateBasket = ref(false)

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(amount)
}

const getStatusColor = (status: string) => {
  const colors: Record<string, 'success' | 'warning' | 'error' | 'info'> = {
    active: 'success',
    archived: 'warning',
    disabled: 'error'
  }
  return colors[status] || 'info'
}

async function loadBaskets() {
  try {
    const response = await basketService.getAllBaskets()
    mainBasket.value = response.data.find(b => b.basket_type === 'main') || null
    branchBaskets.value = response.data.filter(b => b.basket_type !== 'main')
  } catch (error) {
    toast.add({ title: 'Failed to load baskets', color: 'error' })
  }
}

function handleBasketCreated() {
  showCreateBasket.value = false
  loadBaskets()
  toast.add({ title: 'Basket created successfully!', color: 'success' })
}

onMounted(() => {
  loadBaskets()
})
</script>
