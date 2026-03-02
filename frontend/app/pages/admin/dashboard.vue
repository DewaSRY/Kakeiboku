<template>
  <div>
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900">Admin Dashboard</h1>
      <p class="text-gray-600 mt-1">System overview and user statistics.</p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
      <DashboardStatCard
        title="Total Users"
        :value="stats.totalUsers.toString()"
        icon="i-heroicons-users"
        color="blue"
      />
      <DashboardStatCard
        title="Active Users"
        :value="stats.activeUsers.toString()"
        icon="i-heroicons-user-group"
        color="green"
      />
      <DashboardStatCard
        title="Total Transactions"
        :value="stats.totalTransactions.toString()"
        icon="i-heroicons-arrow-path"
        color="amber"
      />
      <DashboardStatCard
        title="Total Volume"
        :value="formatCurrency(stats.totalVolume)"
        icon="i-heroicons-banknotes"
        color="purple"
      />
    </div>

    <!-- Admin Content -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <UCard>
        <template #header>
          <h2 class="text-lg font-semibold text-gray-900">User Statistics</h2>
        </template>
        <div class="space-y-4">
          <div class="flex justify-between items-center py-2 border-b">
            <span class="text-gray-600">New Users (This Month)</span>
            <span class="font-semibold">{{ stats.newUsersThisMonth }}</span>
          </div>
          <div class="flex justify-between items-center py-2 border-b">
            <span class="text-gray-600">Average Baskets Per User</span>
            <span class="font-semibold">{{ stats.avgBasketsPerUser }}</span>
          </div>
          <div class="flex justify-between items-center py-2">
            <span class="text-gray-600">Users with Transactions</span>
            <span class="font-semibold">{{ stats.usersWithTransactions }}</span>
          </div>
        </div>
      </UCard>

      <UCard>
        <template #header>
          <h2 class="text-lg font-semibold text-gray-900">Transaction Metrics</h2>
        </template>
        <div class="space-y-4">
          <div class="flex justify-between items-center py-2 border-b">
            <span class="text-gray-600">Transactions Today</span>
            <span class="font-semibold">{{ stats.transactionsToday }}</span>
          </div>
          <div class="flex justify-between items-center py-2 border-b">
            <span class="text-gray-600">Average Transaction Amount</span>
            <span class="font-semibold">{{ formatCurrency(stats.avgTransactionAmount) }}</span>
          </div>
          <div class="flex justify-between items-center py-2">
            <span class="text-gray-600">Peak Transaction Hour</span>
            <span class="font-semibold">{{ stats.peakHour }}</span>
          </div>
        </div>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'dashboard',
  middleware: 'auth'
})

// Mock admin stats - in production these would come from an API
const stats = reactive({
  totalUsers: 1250,
  activeUsers: 892,
  totalTransactions: 15420,
  totalVolume: 2450000,
  newUsersThisMonth: 124,
  avgBasketsPerUser: 4.2,
  usersWithTransactions: 756,
  transactionsToday: 342,
  avgTransactionAmount: 158.75,
  peakHour: '2:00 PM - 3:00 PM'
})

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 0,
    maximumFractionDigits: 0
  }).format(amount)
}
</script>
