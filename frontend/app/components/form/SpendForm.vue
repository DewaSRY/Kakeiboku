<template>
  <FormContainer :title="$t('dashboard.spendTitle')">
    <UForm :schema="SpendPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UFormField :label="$t('transactions.fromBasket')" name="from_basket_id">
        <USelect
          v-model="formState.from_basket_id"
          :items="branchBaskets"
          value-key="id"
          label-key="name"
          :placeholder="$t('transactions.selectSource')"
          size="lg"
        />
      </UFormField>

      <UFormField :label="$t('common.amount')" name="amount">
        <UInput 
          v-model.number="formState.amount" 
          type="number"
          :placeholder="$t('common.amount')"
          size="lg"
          step="0.01"
          min="0.01"
        />
      </UFormField>

      <UFormField :label="$t('transactions.transactionType')" name="transaction_type_id">
        <USelect
          v-model="formState.transaction_type_id"
          :items="transactionTypes"
          value-key="id"
          label-key="name"
          :placeholder="$t('transactions.selectType')"
          size="lg"
        />
      </UFormField>

      <UiTextInputUi
        v-model="formState.title"
        :label="$t('common.title')"
        name="title"
        :placeholder="$t('common.title')"
        icon="i-lucide-file-text"
      />

      <TextAreaInput
        v-model="formState.description"
        :label="$t('common.description')"
        name="description"
        :placeholder="$t('transactions.optionalNotes')"
        icon="i-lucide-file-text"
        :rows="3"
      />

      <FormActions 
        :submit-text="$t('dashboard.spend')" 
        :loading="isLoading"
        @cancel="$emit('cancel')"
      />
    </UForm>

    <FormError :error="error" />
  </FormContainer>
</template>

<script setup lang="ts">
import { SpendPayloadSchema, type SpendPayload, type BasketResponse, type IdNameResponse } from '~/dtos'
import { useTransactions, useBaskets, useCommonData } from '#imports'
import TextAreaInput from '../ui/TextAreaInput.vue';

const emit = defineEmits<{
  success: []
  cancel: []
}>()

const { t } = useI18n()
const transactionService = useTransactions()
const basketService = useBaskets()
const commonService = useCommonData()

const formState = reactive<SpendPayload>({
  from_basket_id: 0,
  amount: 0,
  transaction_type_id: 0,
  title: '',
  description: undefined
})

const branchBaskets = ref<(Omit<BasketResponse, 'description'> & { description?: string })[]>([])
const transactionTypes = ref<IdNameResponse[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

async function loadFormData() {
  try {
    const [basketsData, typesData] = await Promise.all([
      basketService.fetchBaskets({ limit: 100, page: 1 }),
      commonService.fetchTransactionTypes()
    ])

    branchBaskets.value = basketsData.data
      .filter(b => b.basket_type !== 'main')
      .map(b => ({ ...b, description: b.description ?? undefined }))
    transactionTypes.value = typesData.data
  } catch (e) {
    error.value = t('form.loadDataFailed')
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await transactionService.spend(formState)
    emit('success')
  } catch (e: any) {
    error.value = e.response?.data?.message || t('form.createTransactionFailed')
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadFormData()
})
</script>
