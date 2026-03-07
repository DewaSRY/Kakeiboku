<template>
  <div class="min-h-screen flex items-center justify-center bg-linear-to-br from-amber-50 via-white to-orange-50 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950 relative overflow-hidden">
    <!-- Decorative blobs -->
    <div class="absolute top-[-10%] left-[-5%] w-96 h-96 bg-amber-300/20 dark:bg-amber-500/10 rounded-full blur-3xl" />
    <div class="absolute bottom-[-10%] right-[-5%] w-80 h-80 bg-orange-200/30 dark:bg-orange-500/5 rounded-full blur-3xl" />
    <div class="absolute top-1/3 right-[10%] animate-float opacity-10">
      <UIcon name="i-lucide-wallet" class="w-24 h-24 text-amber-500" />
    </div>
    <div class="absolute bottom-1/4 left-[8%] animate-float-delayed opacity-10">
      <UIcon name="i-lucide-piggy-bank" class="w-20 h-20 text-orange-400" />
    </div>

    <div class="absolute top-4 right-4 flex items-center gap-2">
      <ColorModeToggle />
      <LanguageSwitcher />
    </div>

    <div class="relative z-10 animate-fade-in-up">
      <slot />
    </div>
  </div>
</template>
