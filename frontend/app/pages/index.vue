<template>
  <div class="min-h-screen">
    <!-- Hero Section -->
    <header class="bg-white shadow-sm">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 bg-amber-400 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold">K</span>
          </div>
          <span class="text-xl font-bold text-gray-900">Kakeibo</span>
        </div>
        <div class="flex items-center space-x-4">
          <NuxtLink to="/about" class="text-gray-600 hover:text-gray-900">
            About
          </NuxtLink>
          <NuxtLink to="/signin">
            <UButton color="neutral" variant="ghost">Sign In</UButton>
          </NuxtLink>
          <NuxtLink to="/signup">
            <UButton color="primary">Get Started</UButton>
          </NuxtLink>
        </div>
      </nav>
    </header>

    <!-- Hero Content -->
    <main class="bg-gradient-to-br from-amber-50 to-white">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
        <div class="text-center">
          <h1 class="text-5xl font-bold text-gray-900 mb-6">
            Master Your Finances with
            <span class="text-amber-500">Kakeibo</span>
          </h1>
          <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
            A traditional Japanese budgeting system for mindful spending. 
            Track your income, allocate to baskets, and achieve your financial goals.
          </p>
          <div class="flex justify-center space-x-4">
            <NuxtLink to="/signup">
              <UButton size="xl" color="primary">Start Free Trial</UButton>
            </NuxtLink>
            <NuxtLink to="/about">
              <UButton size="xl" color="neutral" variant="outline">Learn More</UButton>
            </NuxtLink>
          </div>
        </div>

        <!-- Features Grid -->
        <div class="mt-24 grid md:grid-cols-3 gap-8">
          <div class="bg-white p-6 rounded-xl shadow-sm">
            <div class="w-12 h-12 bg-amber-100 rounded-lg flex items-center justify-center mb-4">
              <UIcon name="i-heroicons-wallet" class="w-6 h-6 text-amber-500" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">Main Wallet</h3>
            <p class="text-gray-600">
              Deposit your income into a central wallet and allocate to different spending categories.
            </p>
          </div>

          <div class="bg-white p-6 rounded-xl shadow-sm">
            <div class="w-12 h-12 bg-amber-100 rounded-lg flex items-center justify-center mb-4">
              <UIcon name="i-heroicons-archive-box" class="w-6 h-6 text-amber-500" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">Branch Baskets</h3>
            <p class="text-gray-600">
              Create separate baskets for food, rent, savings, and more. Budget intentionally.
            </p>
          </div>

          <div class="bg-white p-6 rounded-xl shadow-sm">
            <div class="w-12 h-12 bg-amber-100 rounded-lg flex items-center justify-center mb-4">
              <UIcon name="i-heroicons-chart-bar" class="w-6 h-6 text-amber-500" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">Track Spending</h3>
            <p class="text-gray-600">
              Monitor all transactions with detailed categories and visual summaries.
            </p>
          </div>
        </div>
      </div>
    </main>

    <!-- Footer -->
    <footer class="bg-gray-900 text-white py-12">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <div class="flex items-center justify-center space-x-2 mb-4">
          <div class="w-8 h-8 bg-amber-400 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold">K</span>
          </div>
          <span class="text-xl font-bold">Kakeibo</span>
        </div>
        <p class="text-gray-400">
          &copy; 2026 Kakeibo. All rights reserved.
        </p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'default'
})
</script>
