<template>
  <UAlert 
    v-if="error" 
    color="error" 
    variant="soft" 
    :title="error"
    class="mt-4"
  />
</template>

<script setup lang="ts">
interface Props {
  error: string | null
}

defineProps<Props>()
</script>
