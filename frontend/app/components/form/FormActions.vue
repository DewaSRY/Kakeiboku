<template>
  <div class="flex justify-end space-x-3 pt-4">
    <UButton 
      v-if="showCancel"
      color="neutral" 
      variant="ghost" 
      @click="$emit('cancel')"
    >
      {{ cancelText }}
    </UButton>
    <UButton 
      type="submit" 
      :color="submitColor" 
      :loading="loading"
    >
      {{ submitText }}
    </UButton>
  </div>
</template>

<script setup lang="ts">
interface Props {
  submitText?: string
  cancelText?: string
  submitColor?: 'primary' | 'error' | 'success' | 'warning' | 'neutral'
  loading?: boolean
  showCancel?: boolean
}

withDefaults(defineProps<Props>(), {
  submitText: 'Submit',
  cancelText: 'Cancel',
  submitColor: 'primary',
  loading: false,
  showCancel: true
})

defineEmits<{
  cancel: []
}>()
</script>
