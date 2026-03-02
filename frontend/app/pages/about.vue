<template>
  <div class="min-h-screen">
    <!-- Header -->
    <header class="bg-white shadow-sm">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <NuxtLink to="/" class="flex items-center space-x-2">
          <div class="w-8 h-8 bg-amber-400 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold">K</span>
          </div>
          <span class="text-xl font-bold text-gray-900">Kakeibo</span>
        </NuxtLink>
        <NuxtLink to="/signin">
          <UButton color="primary">Sign In</UButton>
        </NuxtLink>
      </nav>
    </header>

    <!-- About Content -->
    <main class="max-w-4xl mx-auto px-4 py-16">
      <h1 class="text-4xl font-bold text-gray-900 mb-8">About Kakeibo</h1>
      
      <div class="prose prose-lg max-w-none">
        <p class="text-gray-600 mb-6">
          <strong>Kakeibo</strong> (家計簿) is the traditional Japanese budgeting method 
          that has helped millions of people gain control of their finances since 1904.
        </p>

        <h2 class="text-2xl font-semibold text-gray-900 mt-8 mb-4">How It Works</h2>
        
        <div class="space-y-6">
          <div class="bg-amber-50 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 mb-2">1. Know Your Income</h3>
            <p class="text-gray-600">
              Start by depositing your income into the main basket. This is your 
              starting point for all financial decisions.
            </p>
          </div>

          <div class="bg-amber-50 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 mb-2">2. Allocate to Baskets</h3>
            <p class="text-gray-600">
              Divide your income into different baskets: needs (rent, utilities), 
              wants (entertainment), savings, and extras.
            </p>
          </div>

          <div class="bg-amber-50 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 mb-2">3. Track Every Transaction</h3>
            <p class="text-gray-600">
              Log every expense with categories. This mindful approach helps you 
              understand your spending habits.
            </p>
          </div>

          <div class="bg-amber-50 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 mb-2">4. Review & Reflect</h3>
            <p class="text-gray-600">
              At the end of each period, review your spending and reflect on 
              what you can improve.
            </p>
          </div>
        </div>

        <div class="mt-12 text-center">
          <NuxtLink to="/signup">
            <UButton size="xl" color="primary">Start Your Journey</UButton>
          </NuxtLink>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'default'
})
</script>
