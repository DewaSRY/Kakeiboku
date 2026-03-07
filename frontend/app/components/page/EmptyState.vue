<template>
  <div :class="['text-center', paddingClass]">
    <UIcon :name="icon" class="w-12 h-12 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
    <p class="text-gray-500 dark:text-gray-400">{{ message }}</p>
    <slot name="action" />
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon?: string
  message: string
  padding?: 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  icon: 'i-lucide-inbox',
  padding: 'md'
})

const paddingClass = computed(() => {
  switch (props.padding) {
    case 'sm': return 'py-6'
    case 'lg': return 'py-16'
    default: return 'py-12'
  }
})
</script>
