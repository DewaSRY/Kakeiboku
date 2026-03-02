<template>
  <div class="bg-white rounded-xl shadow-sm p-6">
    <div class="flex items-center justify-between">
      <div>
        <p class="text-sm text-gray-500">{{ title }}</p>
        <h3 class="text-2xl font-bold text-gray-900 mt-1">{{ value }}</h3>
      </div>
      <div class="w-12 h-12 rounded-lg flex items-center justify-center" :class="bgColorClass">
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
  amber: { bg: 'bg-amber-100', icon: 'text-amber-500' },
  green: { bg: 'bg-green-100', icon: 'text-green-500' },
  blue: { bg: 'bg-blue-100', icon: 'text-blue-500' },
  purple: { bg: 'bg-purple-100', icon: 'text-purple-500' },
  red: { bg: 'bg-red-100', icon: 'text-red-500' }
}

const bgColorClass = computed(() => colorMap[props.color]?.bg || colorMap.amber.bg)
const iconColorClass = computed(() => colorMap[props.color]?.icon || colorMap.amber.icon)
</script>
