<template>
  <div class="text-center py-12">
    <UIcon :name="icon" class="w-12 h-12 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
    <p class="text-gray-500 dark:text-gray-400">{{ message || $t('common.noData') }}</p>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon?: string
  message?: string
}

withDefaults(defineProps<Props>(), {
  icon: 'i-heroicons-document-text'
})
</script>
