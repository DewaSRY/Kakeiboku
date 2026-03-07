<template>
  <UFormField
    :label="label"
    :name="name"
    :required="required"
    :help="help"
  >
    <UTextarea
      :model-value="modelValue"
      :placeholder="placeholder"
      :name="name"
      :size="size"
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
  help?: string
  rows?: number
}

withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: 'Description',
  name: 'description',
  placeholder: 'Enter description',
  required: false,
  disabled: false,
  size: 'xl',
  help: '',
  rows: 4,
})

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>