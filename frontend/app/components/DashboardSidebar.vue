<template>
  <aside class="fixed left-0 top-16 bottom-0 w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 z-30">
    <nav class="p-4 space-y-2">
      <NuxtLink
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="flex items-center space-x-3 px-4 py-3 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-amber-50 dark:hover:bg-amber-900/20 hover:text-amber-600 dark:hover:text-amber-400 transition-colors"
        :class="{ 'bg-amber-50 dark:bg-amber-900/20 text-amber-600 dark:text-amber-400': isActive(item.path) }"
      >
        <UIcon :name="item.icon" class="w-5 h-5" />
        <span class="font-medium">{{ item.label }}</span>
      </NuxtLink>
    </nav>

    <div class="absolute bottom-0 left-0 right-0 p-4 border-t border-gray-200 dark:border-gray-700">
      <div class="text-xs text-gray-400 dark:text-gray-500 text-center">
        Kakeibo v1.0.0
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
const route = useRoute()

const navItems = [
  { path: '/user/dashboard', label: 'Dashboard', icon: 'i-heroicons-home' },
  { path: '/user/baskets', label: 'Baskets', icon: 'i-heroicons-archive-box' },
  { path: '/user/transactions', label: 'Transactions', icon: 'i-heroicons-arrow-path' },
  { path: '/admin/dashboard', label: 'Admin', icon: 'i-heroicons-shield-check' }
]

const isActive = (path: string) => {
  return route.path === path || route.path.startsWith(path + '/')
}
</script>
