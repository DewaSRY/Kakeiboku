<template>
  <UButton
    :icon="colorMode.value === 'dark' ? 'i-lucide-sun' : 'i-lucide-moon'"
    color="neutral"
    variant="ghost"
    @click="toggleColorMode"
  />
</template>

<script setup lang="ts">
const colorMode = useColorMode()

function toggleColorMode() {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}
</script>
