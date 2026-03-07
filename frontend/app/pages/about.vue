<template>
  <div class="min-h-screen bg-white dark:bg-gray-950 transition-colors">
    <!-- Header -->
    <header class="fixed top-0 inset-x-0 z-50 glass">
      <nav class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <NuxtLink to="/" class="flex items-center gap-2.5 group">
          <div class="w-9 h-9 bg-linear-to-br from-amber-400 to-amber-500 rounded-xl flex items-center justify-center shadow-lg shadow-amber-400/25 group-hover:shadow-amber-400/40 transition-shadow">
            <span class="text-white font-bold text-lg">K</span>
          </div>
          <span class="text-xl font-bold text-gray-900 dark:text-white">{{ $t('common.appName') }}</span>
        </NuxtLink>
        <div class="flex items-center gap-3">
          <ColorModeToggle />
          <LanguageSwitcher />
          <NuxtLink to="/signin">
            <UButton color="primary" size="sm">{{ $t('common.signIn') }}</UButton>
          </NuxtLink>
        </div>
      </nav>
    </header>

    <!-- Content -->
    <main class="relative overflow-hidden">
      <div class="absolute inset-0 bg-linear-to-b from-amber-50/50 via-white to-white dark:from-gray-900 dark:via-gray-950 dark:to-gray-950" />
      <div class="absolute top-20 right-0 w-96 h-96 bg-amber-200/20 dark:bg-amber-500/5 rounded-full blur-3xl" />

      <div class="relative max-w-4xl mx-auto px-4 pt-28 pb-16">
        <div class="animate-fade-in-up">
          <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-amber-100/80 dark:bg-amber-900/30 border border-amber-200 dark:border-amber-800/50 mb-6">
            <UIcon name="i-lucide-info" class="w-4 h-4 text-amber-600 dark:text-amber-400" />
            <span class="text-sm font-medium text-amber-700 dark:text-amber-300">{{ $t('about.title') }}</span>
          </div>

          <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-6">
            {{ $t('about.title') }}
          </h1>

          <p class="text-lg text-gray-600 dark:text-gray-400 mb-12 leading-relaxed max-w-3xl">
            <strong class="text-gray-900 dark:text-white">{{ $t('common.appName') }}</strong> {{ $t('about.intro', { appName: '' }) }}
          </p>
        </div>

        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-8">{{ $t('about.howItWorks') }}</h2>

        <!-- Steps Timeline -->
        <div class="space-y-6 stagger-children mb-16">
          <div class="group flex gap-5 items-start">
            <div class="shrink-0 w-12 h-12 rounded-2xl bg-linear-to-br from-amber-400 to-amber-500 flex items-center justify-center text-white font-bold text-lg shadow-lg shadow-amber-400/25">1</div>
            <div class="flex-1 bg-white dark:bg-gray-900 p-6 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.knowIncome.title') }}</h3>
              <p class="text-gray-600 dark:text-gray-400 leading-relaxed">{{ $t('about.steps.knowIncome.description') }}</p>
            </div>
          </div>

          <div class="group flex gap-5 items-start">
            <div class="shrink-0 w-12 h-12 rounded-2xl bg-linear-to-br from-emerald-400 to-emerald-500 flex items-center justify-center text-white font-bold text-lg shadow-lg shadow-emerald-400/25">2</div>
            <div class="flex-1 bg-white dark:bg-gray-900 p-6 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.allocate.title') }}</h3>
              <p class="text-gray-600 dark:text-gray-400 leading-relaxed">{{ $t('about.steps.allocate.description') }}</p>
            </div>
          </div>

          <div class="group flex gap-5 items-start">
            <div class="shrink-0 w-12 h-12 rounded-2xl bg-linear-to-br from-violet-400 to-violet-500 flex items-center justify-center text-white font-bold text-lg shadow-lg shadow-violet-400/25">3</div>
            <div class="flex-1 bg-white dark:bg-gray-900 p-6 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.track.title') }}</h3>
              <p class="text-gray-600 dark:text-gray-400 leading-relaxed">{{ $t('about.steps.track.description') }}</p>
            </div>
          </div>

          <div class="group flex gap-5 items-start">
            <div class="shrink-0 w-12 h-12 rounded-2xl bg-linear-to-br from-rose-400 to-rose-500 flex items-center justify-center text-white font-bold text-lg shadow-lg shadow-rose-400/25">4</div>
            <div class="flex-1 bg-white dark:bg-gray-900 p-6 rounded-2xl border border-gray-100 dark:border-gray-800 card-hover">
              <h3 class="font-bold text-gray-900 dark:text-white mb-2">{{ $t('about.steps.review.title') }}</h3>
              <p class="text-gray-600 dark:text-gray-400 leading-relaxed">{{ $t('about.steps.review.description') }}</p>
            </div>
          </div>
        </div>

        <div class="text-center">
          <NuxtLink to="/signup">
            <UButton size="xl" color="primary" class="shadow-lg shadow-amber-400/25 hover:shadow-amber-400/40 transition-shadow">
              <UIcon name="i-lucide-rocket" class="w-5 h-5 mr-2" />
              {{ $t('about.startJourney') }}
            </UButton>
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
