<template>
  <FormContainer :title="$t('transactions.createTransaction')">
    <UForm :schema="CreateTransactionPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UFormField :label="$t('common.title')" name="title">
        <UInput 
          v-model="formState.title" 
          placeholder="e.g., Grocery shopping"
          size="lg"
        />
      </UFormField>

      <div class="grid grid-cols-2 gap-4">
        <UFormField :label="$t('transactions.fromBasket')" name="from_basket_id">
          <USelect
            v-model="formState.from_basket_id"
            :items="basketOptions"
            :placeholder="$t('transactions.selectSource')"
            size="lg"
          />
        </UFormField>

        <UFormField :label="$t('transactions.toBasket')" name="to_basket_id">
          <USelect
            v-model="formState.to_basket_id"
            :items="basketOptions"
            :placeholder="$t('transactions.selectDestination')"
            size="lg"
          />
        </UFormField>
      </div>

      <UFormField :label="$t('common.amount')" name="amount">
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

      <UFormField :label="$t('transactions.transactionType')" name="transaction_type_id">
        <USelect
          v-model="formState.transaction_type_id"
          :items="transactionTypeOptions"
          :placeholder="$t('transactions.selectType')"
          size="lg"
        />
      </UFormField>

      <UFormField :label="$t('common.description')" name="description">
        <UTextarea 
          v-model="formState.description" 
          :placeholder="$t('transactions.optionalNotes')"
          :rows="3"
        />
      </UFormField>

      <FormActions 
        :submit-text="$t('transactions.createTransaction')" 
        :loading="isLoading"
        @cancel="$emit('cancel')"
      />
    </UForm>

    <FormError :error="error" />
  </FormContainer>
</template>

<script setup lang="ts">
import { CreateTransactionPayloadSchema, type CreateTransactionPayload, type IdNameResponse, type BasketResponse } from '~/dtos'
import { useTransactions, useCommonData, useBaskets } from '#imports'


const emit = defineEmits<{
  success: []
  cancel: []
}>()

const { t } = useI18n()
const transactionService = useTransactions()
const commonService = useCommonData()
const basketService = useBaskets()

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

const basketOptions = computed(() => 
  baskets.value.map(b => ({ value: b.id, label: b.name }))
)

const transactionTypeOptions = computed(() => 
  transactionTypes.value.map(t => ({ value: t.id, label: t.name }))
)

async function loadData() {
  try {
    const [basketsRes, typesRes] = await Promise.all([
      basketService.fetchBaskets(),
      commonService.fetchTransactionTypes()
    ])
    baskets.value = basketsRes.data
    transactionTypes.value = typesRes.data
  } catch (e) {
    error.value = t('form.loadDataFailed')
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await transactionService.createTransaction(formState)
    emit('success')
  } catch (e: any) {
    error.value = e.response?.data?.message || t('form.createTransactionFailed')
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadData()
})
</script>
