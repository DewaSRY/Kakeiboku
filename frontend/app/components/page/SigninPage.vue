<template>
  <div
    class="w-full max-w-md p-8 bg-white dark:bg-gray-800 rounded-2xl shadow-xl dark:shadow-gray-900/50"
  >
    <div class="text-center mb-8">
      <div
        class="w-12 h-12 bg-amber-400 rounded-xl flex items-center justify-center mx-auto mb-4"
      >
        <span class="text-white font-bold text-xl">K</span>
      </div>
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        {{ $t("auth.welcomeBack") }}
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mt-2">
        {{ $t("auth.signInSubtitle") }}
      </p>
    </div>

    <SigninForm />

    <div class="mt-6 text-center">
      <p class="text-gray-600 dark:text-gray-400">
        {{ $t("auth.noAccount") }}
        <NuxtLink
          to="/signup"
          class="text-amber-500 hover:text-amber-600 font-medium"
        >
          {{ $t("common.signUp") }}
        </NuxtLink>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import SigninForm from "../form/SigninForm.vue";
</script>
