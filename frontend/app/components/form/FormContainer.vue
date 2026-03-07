<template>
  <div class="p-6">
    <h2 v-if="title" class="text-xl font-extrabold text-gray-900 dark:text-white mb-6">{{ title }}</h2>
    <slot />
  </div>
</template>

<script setup lang="ts">
interface Props {
  title?: string
}

defineProps<Props>()
</script>
