<template>
  <UFormField
    :label="label"
    :name="name"
    :required="required"
    :help="help"
  >
    <USelect
      :model-value="modelValue"
      :items="items"
      :placeholder="placeholder"
      :name="name"
      :size="size"
      :icon="icon"
      :disabled="disabled"
      :value-key="valueKey"
      :label-key="labelKey"
      @update:model-value="handleChange"
      class="w-full"
    />
  </UFormField>
</template>

<script setup lang="ts" generic="T">

export interface SelectItem {
  [key: string]: unknown
}

interface Props {
  modelValue?: T
  items: SelectItem[]
  label?: string
  name?: string
  placeholder?: string
  required?: boolean
  disabled?: boolean
  size?: 'sm' | 'md' | 'lg' | 'xl'
  icon?: string
  help?: string
  valueKey?: string
  labelKey?: string
}

withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  label: 'Select',
  name: 'select',
  placeholder: 'Select an option',
  required: false,
  disabled: false,
  size: 'lg',
  icon: undefined,
  help: '',
  valueKey: 'id',
  labelKey: 'name',
})

const emit = defineEmits<{
  'update:modelValue': [value: T]
  'change': [value: T]
}>()

function handleChange(value: unknown) {
  emit('update:modelValue', value as T)
  emit('change', value as T)
}
</script>
