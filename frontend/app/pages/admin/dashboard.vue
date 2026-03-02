<template>
  <div>
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{{ $t('admin.title') }}</h1>
      <p class="text-gray-600 dark:text-gray-400 mt-1">{{ $t('admin.subtitle') }}</p>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
      <DashboardStatCard
        :title="$t('admin.totalUsers')"
        :value="stats.totalUsers.toString()"
        icon="i-heroicons-users"
        color="blue"
      />
      <DashboardStatCard
        :title="$t('admin.activeUsers')"
        :value="stats.activeUsers.toString()"
        icon="i-heroicons-user-group"
        color="green"
      />
      <DashboardStatCard
        :title="$t('admin.totalTransactions')"
        :value="stats.totalTransactions.toString()"
        icon="i-heroicons-arrow-path"
        color="amber"
      />
      <DashboardStatCard
        :title="$t('admin.totalVolume')"
        :value="formatCurrency(stats.totalVolume)"
        icon="i-heroicons-banknotes"
        color="purple"
      />
    </div>

    <!-- Admin Content -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <ContentCard :title="$t('admin.userStatistics')">
        <div class="space-y-4">
          <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
            <span class="text-gray-600 dark:text-gray-400">{{ $t('admin.newUsersThisMonth') }}</span>
            <span class="font-semibold text-gray-900 dark:text-white">{{ stats.newUsersThisMonth }}</span>
          </div>
          <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
            <span class="text-gray-600 dark:text-gray-400">{{ $t('admin.avgBasketsPerUser') }}</span>
            <span class="font-semibold text-gray-900 dark:text-white">{{ stats.avgBasketsPerUser }}</span>
          </div>
          <div class="flex justify-between items-center py-2">
            <span class="text-gray-600 dark:text-gray-400">{{ $t('admin.usersWithTransactions') }}</span>
            <span class="font-semibold text-gray-900 dark:text-white">{{ stats.usersWithTransactions }}</span>
          </div>
        </div>
      </ContentCard>

      <ContentCard :title="$t('admin.transactionMetrics')">
        <div class="space-y-4">
          <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
            <span class="text-gray-600 dark:text-gray-400">{{ $t('admin.transactionsToday') }}</span>
            <span class="font-semibold text-gray-900 dark:text-white">{{ stats.transactionsToday }}</span>
          </div>
          <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
            <span class="text-gray-600 dark:text-gray-400">{{ $t('admin.avgTransactionAmount') }}</span>
            <span class="font-semibold text-gray-900 dark:text-white">{{ formatCurrency(stats.avgTransactionAmount) }}</span>
          </div>
          <div class="flex justify-between items-center py-2">
            <span class="text-gray-600 dark:text-gray-400">{{ $t('admin.peakTransactionHour') }}</span>
            <span class="font-semibold text-gray-900 dark:text-white">{{ stats.peakHour }}</span>
          </div>
        </div>
      </ContentCard>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'dashboard',
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
