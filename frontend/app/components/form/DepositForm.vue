<template>
  <FormContainer :title="$t('dashboard.depositTitle')">
    <UForm :schema="DepositPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UiNumberInput
        v-model="formState.amount"
        :label="$t('common.amount')"
        name="amount"
        :placeholder="$t('common.amount')"
        mask="currency-idr"
        icon="i-lucide-banknote"
        required
      />

      <UiSelectOption
        v-model="formState.transaction_type_id"
        :items="transactionTypes"
        :label="$t('transactions.transactionType')"
        name="transaction_type_id"
        :placeholder="$t('transactions.selectType')"
        required
      />

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
        :rows="3"
      />

      <FormActions 
        :submit-text="$t('dashboard.deposite')" 
        :loading="isLoading"
        @cancel="$emit('cancel')"
      />
    </UForm>

    <FormError :error="error" />
  </FormContainer>
</template>

<script setup lang="ts">
import { DepositPayloadSchema, type DepositPayload, type IdNameResponse } from '~/dtos'
import { useTransactions, useCommonData } from '#imports'
import TextAreaInput from '../ui/TextAreaInput.vue';

const emit = defineEmits<{
  success: []
  cancel: []
}>()

const { t } = useI18n()
const transactionService = useTransactions()
const commonService = useCommonData()

const formState = reactive<Partial<DepositPayload>>({
  amount: undefined,
  transaction_type_id: undefined,
  title: '',
  description: undefined
})

const transactionTypes = ref<IdNameResponse[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

async function loadTransactionTypes() {
  try {
    const response = await commonService.fetchTransactionTypes()
    transactionTypes.value = response.data
  } catch (e) {
    error.value = t('form.loadDataFailed')
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await transactionService.deposit(formState as DepositPayload)
    emit('success')
  } catch (e: any) {
    error.value = e.response?.data?.message || t('form.createTransactionFailed')
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadTransactionTypes()
})
</script>
