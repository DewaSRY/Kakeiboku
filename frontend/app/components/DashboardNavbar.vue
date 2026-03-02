<template>
  <header class="fixed top-0 left-0 right-0 h-16 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 z-40">
    <div class="h-full px-6 flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <NuxtLink to="/" class="flex items-center space-x-2">
          <div class="w-8 h-8 bg-amber-400 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold">K</span>
          </div>
          <span class="text-xl font-bold text-gray-900 dark:text-white">Kakeibo</span>
        </NuxtLink>
      </div>

      <div class="flex items-center space-x-4">
        <ColorModeToggle />
        
        <UButton 
          color="neutral" 
          variant="ghost" 
          icon="i-heroicons-bell"
          class="relative"
        />
        
        <UDropdownMenu 
          :items="userMenuItems"
          :content="{ align: 'end' }"
        >
          <UButton 
            color="neutral" 
            variant="ghost"
            class="flex items-center space-x-2"
          >
            <UAvatar size="sm" :alt="userName" />
            <span class="hidden md:block">{{ userName }}</span>
            <UIcon name="i-heroicons-chevron-down" class="w-4 h-4" />
          </UButton>
        </UDropdownMenu>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useAuthService } from '~/services'

const authService = useAuthService()

const userName = ref('User')

const userMenuItems = [
  [
    { label: 'Profile', icon: 'i-heroicons-user', click: () => navigateTo('/user/profile') },
    { label: 'Settings', icon: 'i-heroicons-cog-6-tooth', click: () => navigateTo('/user/settings') }
  ],
  [
    { label: 'Sign Out', icon: 'i-heroicons-arrow-right-on-rectangle', click: () => authService.logout() }
  ]
]
</script>
