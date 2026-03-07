<template>
  <UFormField
    :label="label"
    :name="name"
    :required="required"
    :help="help"
  >
    <UInput
      :model-value="displayValue"
      type="text"
      inputmode="numeric"
      :placeholder="placeholder"
      :name="name"
      :size="size"
      :icon="icon"
      :disabled="disabled"
      @input="handleInput"
      @blur="handleBlur"
      @focus="handleFocus"
      class="w-full"
    />
  </UFormField>
</template>

<script setup lang="ts">
type MaskType = 'none' | 'currency-idr' | 'thousand'

interface Props {
  modelValue?: number
  label?: string
  name?: string
  placeholder?: string
  required?: boolean
  disabled?: boolean
  size?: 'sm' | 'md' | 'lg' | 'xl'
  icon?: string
  help?: string
  mask?: MaskType
  min?: number
  max?: number
  step?: number
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  label: 'Amount',
  name: 'amount',
  placeholder: 'Enter amount',
  required: false,
  disabled: false,
  size: 'lg',
  icon: undefined,
  help: '',
  mask: 'none',
  min: undefined,
  max: undefined,
  step: 1
})

const emit = defineEmits<{
  'update:modelValue': [value: number | undefined]
  'change': [value: number | undefined]
}>()

const isFocused = ref(false)

function formatToIDR(value: number): string {
  return new Intl.NumberFormat('id-ID', {
    style: 'currency',
    currency: 'IDR',
    minimumFractionDigits: 0,
    maximumFractionDigits: 0
  }).format(value)
}

function formatThousand(value: number): string {
  return new Intl.NumberFormat('id-ID', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 2
  }).format(value)
}

function formatValue(value: number | undefined): string {
  if (value === undefined || value === null || isNaN(value)) return ''
  
  switch (props.mask) {
    case 'currency-idr':
      return formatToIDR(value)
    case 'thousand':
      return formatThousand(value)
    default:
      return String(value)
  }
}

// Parse formatted string back to number
function parseValue(input: string): number | undefined {
  if (!input || input.trim() === '') return undefined
  
  // Remove currency symbol, dots (thousand separator), and replace comma with dot for decimal
  const cleaned = input
    .replace(/[Rp\s]/g, '')
    .replace(/\./g, '')
    .replace(',', '.')
  
  const parsed = parseFloat(cleaned)
  return isNaN(parsed) ? undefined : parsed
}

const displayValue = computed(() => {
  if (isFocused.value) {
    // Show raw number when focused for easier editing
    return props.modelValue !== undefined ? String(props.modelValue) : ''
  }
  return formatValue(props.modelValue)
})

function handleInput(event: Event) {
  const input = event.target as HTMLInputElement
  let value = input.value
  
  // Allow only numbers, dots, and commas during input
  value = value.replace(/[^\d.,]/g, '')
  
  const parsed = parseValue(value)
  emit('update:modelValue', parsed)
}

function handleFocus() {
  isFocused.value = true
}

function handleBlur() {
  isFocused.value = false
  emit('change', props.modelValue)
}
</script>