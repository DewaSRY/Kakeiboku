<template>
  <FormContainer :title="$t('dashboard.depositTitle')">
    <UForm :schema="DepositPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
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

      <UFormField :label="$t('common.title')" name="title">
        <UInput 
          v-model="formState.title" 
          :placeholder="$t('common.title')"
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

const emit = defineEmits<{
  success: []
  cancel: []
}>()

const { t } = useI18n()
const transactionService = useTransactions()
const commonService = useCommonData()

const formState = reactive<DepositPayload>({
  amount: 0,
  transaction_type_id: 0,
  title: '',
  description: null
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
    await transactionService.deposit(formState)
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
