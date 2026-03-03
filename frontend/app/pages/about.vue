<template>
  <div class="min-h-screen bg-white dark:bg-gray-900 transition-colors">
    <!-- Header -->
    <header class="bg-white dark:bg-gray-800 shadow-sm dark:shadow-gray-900/50">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <NuxtLink to="/" class="flex items-center space-x-2">
          <div class="w-8 h-8 bg-amber-400 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold">K</span>
          </div>
          <span class="text-xl font-bold text-gray-900 dark:text-white">{{ $t('common.appName') }}</span>
        </NuxtLink>
        <div class="flex items-center space-x-4">
          <ColorModeToggle />
          <LanguageSwitcher />
          <NuxtLink to="/signin">
            <UButton color="primary">{{ $t('common.signIn') }}</UButton>
          </NuxtLink>
        </div>
      </nav>
    </header>

    <!-- About Content -->
    <main class="max-w-4xl mx-auto px-4 py-16">
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-8">{{ $t('about.title') }}</h1>
      
      <div class="prose prose-lg dark:prose-invert max-w-none">
        <p class="text-gray-600 dark:text-gray-300 mb-6">
          <strong class="text-gray-900 dark:text-white">{{ $t('common.appName') }}</strong> {{ $t('about.intro', { appName: '' }) }}
        </p>

        <h2 class="text-2xl font-semibold text-gray-900 dark:text-white mt-8 mb-4">{{ $t('about.howItWorks') }}</h2>
        
        <div class="space-y-6">
          <div class="bg-amber-50 dark:bg-amber-900/20 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.knowIncome.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-300">
              {{ $t('about.steps.knowIncome.description') }}
            </p>
          </div>

          <div class="bg-amber-50 dark:bg-amber-900/20 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.allocate.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-300">
              {{ $t('about.steps.allocate.description') }}
            </p>
          </div>

          <div class="bg-amber-50 dark:bg-amber-900/20 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.track.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-300">
              {{ $t('about.steps.track.description') }}
            </p>
          </div>

          <div class="bg-amber-50 dark:bg-amber-900/20 p-6 rounded-lg">
            <h3 class="font-semibold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.review.title') }}</h3>
            <p class="text-gray-600 dark:text-gray-300">
              {{ $t('about.steps.review.description') }}
            </p>
          </div>
        </div>

        <div class="mt-12 text-center">
          <NuxtLink to="/signup">
            <UButton size="xl" color="primary">{{ $t('about.startJourney') }}</UButton>
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
