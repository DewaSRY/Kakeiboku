<template>
  <UCard>
    <template v-if="title" #header>
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">{{ title }}</h2>
        <slot name="header-action" />
      </div>
    </template>
    <slot />
  </UCard>
</template>

<script setup lang="ts">
interface Props {
  title?: string
}

defineProps<Props>()
</script>
