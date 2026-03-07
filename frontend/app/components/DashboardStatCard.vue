<template>
  <div class="bg-white dark:bg-gray-900 rounded-2xl border border-gray-100 dark:border-gray-800 p-6 card-hover">
    <div class="flex items-center justify-between">
      <div>
        <p class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">{{ title }}</p>
        <h3 class="text-2xl font-extrabold text-gray-900 dark:text-white mt-2">{{ value }}</h3>
      </div>
      <div class="w-12 h-12 rounded-2xl flex items-center justify-center" :class="bgColorClass">
        <UIcon :name="icon" class="w-6 h-6" :class="iconColorClass" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  value: string
  icon: string
  color: 'amber' | 'green' | 'blue' | 'purple' | 'red'
}

const props = defineProps<Props>()

const colorMap = {
  amber: { bg: 'bg-amber-100 dark:bg-amber-900/30', icon: 'text-amber-500' },
  green: { bg: 'bg-green-100 dark:bg-green-900/30', icon: 'text-green-500' },
  blue: { bg: 'bg-blue-100 dark:bg-blue-900/30', icon: 'text-blue-500' },
  purple: { bg: 'bg-purple-100 dark:bg-purple-900/30', icon: 'text-purple-500' },
  red: { bg: 'bg-red-100 dark:bg-red-900/30', icon: 'text-red-500' }
}

const bgColorClass = computed(() => colorMap[props.color]?.bg || colorMap.amber.bg)
const iconColorClass = computed(() => colorMap[props.color]?.icon || colorMap.amber.icon)
</script>
