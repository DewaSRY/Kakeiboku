<template>
  <div class="w-full max-w-md p-8 bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl rounded-3xl shadow-2xl shadow-gray-900/10 dark:shadow-black/30 border border-white/50 dark:border-gray-800">
    <div class="text-center mb-8">
      <div class="w-14 h-14 bg-linear-to-br from-amber-400 to-amber-500 rounded-2xl flex items-center justify-center mx-auto mb-5 shadow-lg shadow-amber-400/25 animate-pulse-glow">
        <span class="text-white font-bold text-2xl">K</span>
      </div>
      <h1 class="text-2xl font-extrabold text-gray-900 dark:text-white">{{ $t('auth.createAccount') }}</h1>
      <p class="text-gray-500 dark:text-gray-400 mt-2">{{ $t('auth.createAccountSubtitle') }}</p>
    </div>

    <SignupForm />

    <div class="mt-6 text-center">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        {{ $t('auth.hasAccount') }}
        <NuxtLink to="/signin" class="text-amber-500 hover:text-amber-600 font-semibold transition-colors">
          {{ $t('common.signIn') }}
        </NuxtLink>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import SignupForm from '../form/SignupForm.vue'

</script>
