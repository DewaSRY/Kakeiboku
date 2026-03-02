<template>
  <div>
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">Dashboard</h1>
      <p class="text-gray-600 dark:text-gray-400 mt-1">Welcome back! Here's your financial overview.</p>
    </div>

    <!-- Balance Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <DashboardStatCard
        title="Total Balance"
        :value="formatCurrency(totalBalance)"
        icon="i-heroicons-banknotes"
        color="amber"
      />
      <DashboardStatCard
        title="Main Wallet"
        :value="formatCurrency(mainBasket?.balance || 0)"
        icon="i-heroicons-wallet"
        color="green"
      />
      <DashboardStatCard
        title="Branch Baskets"
        :value="branchBaskets.length.toString()"
        icon="i-heroicons-archive-box"
        color="blue"
      />
    </div>

    <!-- Main Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Branch Baskets -->
      <ContentCard title="Branch Baskets">
        <template #header-action>
          <UButton 
            color="primary" 
            size="sm" 
            icon="i-heroicons-plus"
            @click="showCreateBasket = true"
          >
            New Basket
          </UButton>
        </template>

        <EmptyState 
          v-if="branchBaskets.length === 0"
          icon="i-heroicons-archive-box"
          message="No branch baskets yet. Create one to get started!"
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
      </ContentCard>

      <!-- Recent Transactions -->
      <ContentCard title="Recent Transactions">
        <template #header-action>
          <UButton 
            color="neutral" 
            variant="ghost" 
            size="sm"
            @click="navigateTo('/user/transactions')"
          >
            View All
          </UButton>
        </template>

        <EmptyState 
          v-if="recentTransactions.length === 0"
          icon="i-heroicons-arrow-path"
          message="No transactions yet."
          padding="sm"
        />

        <div v-else class="space-y-4">
          <TransactionItem
            v-for="transaction in recentTransactions"
            :key="transaction.id"
            :transaction="transaction"
          />
        </div>
      </ContentCard>
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
  </div>
</template>

<script setup lang="ts">
import type { BasketResponse, TransactionResponse } from '~/dtos'
import { useBasketService, useTransactionService } from '~/services'

definePageMeta({
  layout: 'dashboard',
})

const basketService = useBasketService()
const transactionService = useTransactionService()
const toast = useToast()

const totalBalance = ref(0)
const mainBasket = ref<BasketResponse | null>(null)
const branchBaskets = ref<BasketResponse[]>([])
const recentTransactions = ref<TransactionResponse[]>([])
const showCreateBasket = ref(false)

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(amount)
}

async function loadDashboardData() {
  try {
    const [basketsResponse, transactions] = await Promise.all([
      basketService.getAllBaskets(),
      transactionService.getRecentTransactions(5)
    ])
    
    const allBaskets = basketsResponse.data
    mainBasket.value = allBaskets.find(b => b.basket_type === 'main') || null
    branchBaskets.value = allBaskets.filter(b => b.basket_type !== 'main')
    totalBalance.value = allBaskets.reduce((acc, b) => acc + b.balance, 0)
    recentTransactions.value = transactions
  } catch (error) {
    toast.add({ title: 'Failed to load dashboard data', color: 'error' })
  }
}

function handleBasketCreated() {
  showCreateBasket.value = false
  loadDashboardData()
  toast.add({ title: 'Basket created successfully!', color: 'success' })
}

onMounted(() => {
  loadDashboardData()
})
</script>
