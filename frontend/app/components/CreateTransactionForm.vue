<template>
  <FormContainer title="New Transaction">
    <UForm :schema="CreateTransactionPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UFormField label="Title" name="title">
        <UInput 
          v-model="formState.title" 
          placeholder="e.g., Grocery shopping"
          size="lg"
        />
      </UFormField>

      <div class="grid grid-cols-2 gap-4">
        <UFormField label="From Basket" name="from_basket_id">
          <USelect
            v-model="formState.from_basket_id"
            :items="baskets"
            value-key="id"
            label-key="name"
            placeholder="Select source"
            size="lg"
          />
        </UFormField>

        <UFormField label="To Basket" name="to_basket_id">
          <USelect
            v-model="formState.to_basket_id"
            :items="baskets"
            value-key="id"
            label-key="name"
            placeholder="Select destination"
            size="lg"
          />
        </UFormField>
      </div>

      <UFormField label="Amount" name="amount">
        <UInput 
          v-model.number="formState.amount" 
          type="number"
          step="0.01"
          min="0"
          placeholder="0.00"
          size="lg"
        >
          <template #leading>
            <span class="text-gray-500">$</span>
          </template>
        </UInput>
      </UFormField>

      <UFormField label="Transaction Type" name="transaction_type_id">
        <USelect
          v-model="formState.transaction_type_id"
          :items="transactionTypes"
          value-key="id"
          label-key="name"
          placeholder="Select type"
          size="lg"
        />
      </UFormField>

      <UFormField label="Description" name="description">
        <UTextarea 
          v-model="formState.description" 
          placeholder="Optional notes..."
          :rows="3"
        />
      </UFormField>

      <FormActions 
        submit-text="Create Transaction" 
        :loading="isLoading"
        @cancel="$emit('cancel')"
      />
    </UForm>

    <FormError :error="error" />
  </FormContainer>
</template>

<script setup lang="ts">
import { CreateTransactionPayloadSchema, type CreateTransactionPayload, type IdNameResponse, type BasketResponse } from '~/dtos'
import { useTransactionService, useCommonService, useBasketService } from '~/services'

const emit = defineEmits<{
  success: []
  cancel: []
}>()

const transactionService = useTransactionService()
const commonService = useCommonService()
const basketService = useBasketService()

const formState = reactive<CreateTransactionPayload>({
  title: '',
  from_basket_id: 0,
  to_basket_id: 0,
  amount: 0,
  transaction_type_id: 0,
  description: null
})

const baskets = ref<BasketResponse[]>([])
const transactionTypes = ref<IdNameResponse[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

async function loadData() {
  try {
    const [basketsRes, typesRes] = await Promise.all([
      basketService.getAllBaskets(),
      commonService.getTransactionTypes()
    ])
    baskets.value = basketsRes.data
    transactionTypes.value = typesRes.data
  } catch (e) {
    error.value = 'Failed to load form data'
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await transactionService.createTransaction(formState)
    emit('success')
  } catch (e: any) {
    error.value = e.response?.data?.message || 'Failed to create transaction'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadData()
})
</script>
