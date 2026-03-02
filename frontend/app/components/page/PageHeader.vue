<template>
  <div class="flex items-center justify-between mb-8">
    <div>
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{{ title }}</h1>
      <p v-if="description" class="text-gray-600 dark:text-gray-400 mt-1">{{ description }}</p>
    </div>
    <slot name="actions">
      <UButton 
        v-if="actionLabel"
        :color="actionColor" 
        :icon="actionIcon"
        @click="$emit('action')"
      >
        {{ actionLabel }}
      </UButton>
    </slot>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  description?: string
  actionLabel?: string
  actionIcon?: string
  actionColor?: 'primary' | 'error' | 'success' | 'warning' | 'neutral'
}

withDefaults(defineProps<Props>(), {
  actionColor: 'primary'
})

defineEmits<{
  action: []
}>()
</script>
