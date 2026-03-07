<template>
  <div class="min-h-screen bg-white dark:bg-gray-950 transition-colors">
    <!-- Navbar -->
    <header class="fixed top-0 inset-x-0 z-50 glass">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <NuxtLink to="/" class="flex items-center gap-2.5 group">
          <div class="w-9 h-9 bg-linear-to-br from-amber-400 to-amber-500 rounded-xl flex items-center justify-center shadow-lg shadow-amber-400/25 group-hover:shadow-amber-400/40 transition-shadow">
            <span class="text-white font-bold text-lg">K</span>
          </div>
          <span class="text-xl font-bold text-gray-900 dark:text-white">{{ $t('common.appName') }}</span>
        </NuxtLink>
        <div class="flex items-center gap-3">
          <NuxtLink to="/about" class="text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-amber-500 dark:hover:text-amber-400 transition-colors px-3 py-2">
            {{ $t('common.about') }}
          </NuxtLink>
          <ColorModeToggle />
          <LanguageSwitcher />
          <NuxtLink to="/signin">
            <UButton color="neutral" variant="ghost" size="sm">{{ $t('common.signIn') }}</UButton>
          </NuxtLink>
          <NuxtLink to="/signup">
            <UButton color="primary" size="sm">{{ $t('common.getStarted') }}</UButton>
          </NuxtLink>
        </div>
      </nav>
    </header>

    <!-- Hero -->
    <main class="relative overflow-hidden">
      <!-- Background decoration -->
      <div class="absolute inset-0 bg-linear-to-br from-amber-50/80 via-white to-orange-50/50 dark:from-gray-900 dark:via-gray-950 dark:to-gray-900" />
      <div class="absolute top-20 left-10 w-72 h-72 bg-amber-300/20 dark:bg-amber-500/10 rounded-full blur-3xl" />
      <div class="absolute bottom-20 right-10 w-96 h-96 bg-orange-200/20 dark:bg-orange-500/5 rounded-full blur-3xl" />

      <!-- Floating decorative icons -->
      <div class="absolute top-32 right-[15%] animate-float opacity-20 dark:opacity-10">
        <UIcon name="i-lucide-wallet" class="w-16 h-16 text-amber-500" />
      </div>
      <div class="absolute top-64 left-[10%] animate-float-delayed opacity-15 dark:opacity-10">
        <UIcon name="i-lucide-piggy-bank" class="w-12 h-12 text-orange-400" />
      </div>
      <div class="absolute bottom-40 right-[25%] animate-float-slow opacity-15 dark:opacity-10">
        <UIcon name="i-lucide-bar-chart-3" class="w-14 h-14 text-amber-600" />
      </div>

      <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-32 pb-24">
        <div class="text-center animate-fade-in-up">
          <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-amber-100/80 dark:bg-amber-900/30 border border-amber-200 dark:border-amber-800/50 mb-8">
            <div class="w-2 h-2 rounded-full bg-amber-400 animate-pulse" />
            <span class="text-sm font-medium text-amber-700 dark:text-amber-300">{{ $t('common.appName') }} &mdash; Smart Budgeting</span>
          </div>

          <h1 class="text-5xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-6 leading-tight">
            {{ $t('home.heroTitle') }}
            <span class="bg-linear-to-r from-amber-400 via-amber-500 to-orange-500 bg-clip-text text-transparent">{{ $t('common.appName') }}</span>
          </h1>

          <p class="text-lg sm:text-xl text-gray-600 dark:text-gray-400 mb-10 max-w-2xl mx-auto leading-relaxed">
            {{ $t('home.heroSubtitle') }}
          </p>

          <div class="flex flex-col sm:flex-row justify-center gap-4">
            <NuxtLink to="/signup">
              <UButton size="xl" color="primary" class="shadow-lg shadow-amber-400/25 hover:shadow-amber-400/40 transition-shadow">
                <UIcon name="i-lucide-rocket" class="w-5 h-5 mr-2" />
                {{ $t('home.startFreeTrial') }}
              </UButton>
            </NuxtLink>
            <NuxtLink to="/about">
              <UButton size="xl" color="neutral" variant="outline">
                <UIcon name="i-lucide-book-open" class="w-5 h-5 mr-2" />
                {{ $t('common.learnMore') }}
              </UButton>
            </NuxtLink>
          </div>
        </div>

        <!-- Features Grid -->
        <div class="mt-28 grid md:grid-cols-3 gap-8 stagger-children">
          <div class="group bg-white dark:bg-gray-900 p-8 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
            <div class="w-14 h-14 bg-linear-to-br from-amber-100 to-amber-200 dark:from-amber-900/40 dark:to-amber-800/30 rounded-2xl flex items-center justify-center mb-5 group-hover:scale-110 transition-transform">
              <UIcon name="i-lucide-wallet" class="w-7 h-7 text-amber-600 dark:text-amber-400" />
            </div>
            <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">{{ $t('home.features.mainWallet.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-400 leading-relaxed">
              {{ $t('home.features.mainWallet.description') }}
            </p>
          </div>

          <div class="group bg-white dark:bg-gray-900 p-8 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
            <div class="w-14 h-14 bg-linear-to-br from-emerald-100 to-emerald-200 dark:from-emerald-900/40 dark:to-emerald-800/30 rounded-2xl flex items-center justify-center mb-5 group-hover:scale-110 transition-transform">
              <UIcon name="i-lucide-archive" class="w-7 h-7 text-emerald-600 dark:text-emerald-400" />
            </div>
            <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">{{ $t('home.features.branchBaskets.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-400 leading-relaxed">
              {{ $t('home.features.branchBaskets.description') }}
            </p>
          </div>

          <div class="group bg-white dark:bg-gray-900 p-8 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
            <div class="w-14 h-14 bg-linear-to-br from-violet-100 to-violet-200 dark:from-violet-900/40 dark:to-violet-800/30 rounded-2xl flex items-center justify-center mb-5 group-hover:scale-110 transition-transform">
              <UIcon name="i-lucide-bar-chart-3" class="w-7 h-7 text-violet-600 dark:text-violet-400" />
            </div>
            <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-2">{{ $t('home.features.trackSpending.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-400 leading-relaxed">
              {{ $t('home.features.trackSpending.description') }}
            </p>
          </div>
        </div>
      </div>
    </main>

    <!-- Footer -->
    <footer class="border-t border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-950 py-12">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <div class="flex items-center justify-center gap-2.5 mb-4">
          <div class="w-8 h-8 bg-linear-to-br from-amber-400 to-amber-500 rounded-xl flex items-center justify-center">
            <span class="text-white font-bold">K</span>
          </div>
          <span class="text-lg font-bold text-gray-900 dark:text-white">{{ $t('common.appName') }}</span>
        </div>
        <p class="text-sm text-gray-500 dark:text-gray-500">
          {{ $t('common.copyright') }}
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
