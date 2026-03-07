<template>
  <UFormField
    :label="label"
    :name="name"
    :required="required"
    :help="help"
  >
    <UInput
      :model-value="modelValue"
      type="text"
      :placeholder="placeholder"
      :name="name"
      :size="size"
      :icon="icon"
      :disabled="disabled"
      :rows="rows"
      @update:model-value="$emit('update:modelValue', $event)"
      class="w-full"
    />
  </UFormField>
</template>

<script setup lang="ts">

interface Props {
  modelValue?: string
  label?: string
  name?: string
  placeholder?: string
  required?: boolean
  disabled?: boolean
  size?: 'sm' | 'md' | 'lg' | 'xl'
  icon?: string
  help?: string
  rows?: number
}

withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: 'Full Name',
  name: 'name',
  placeholder: 'Enter your full name',
  required: false,
  disabled: false,
  size: 'xl',
  icon: 'i-lucide-user',
  help: '',
  rows: 3,
})

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>
