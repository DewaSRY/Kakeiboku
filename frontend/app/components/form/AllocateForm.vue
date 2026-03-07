<template>
  <FormContainer :title="$t('dashboard.allocateTitle')">
    <UForm
      :schema="AllocatePayloadSchema"
      :state="formState"
      @submit="handleSubmit"
      class="space-y-4"
    >
      <UiSelectOption
        v-model="formState.to_basket_id"
        :items="branchBaskets"
        :label="$t('transactions.toBasket')"
        name="to_basket_id"
        :placeholder="$t('transactions.selectDestination')"
        required
      />

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
        :placeholder="$t('common.optionalNotes')"
        icon="i-lucide-file-text"
        :rows="3"
      />

      <FormActions
        :submit-text="$t('dashboard.alocated')"
        :loading="isLoading"
        @cancel="$emit('cancel')"
      />
    </UForm>

    <FormError :error="error" />
  </FormContainer>
</template>

<script setup lang="ts">
import {
  AllocatePayloadSchema,
  type AllocatePayload,
  type BasketResponse,
  type IdNameResponse,
} from "~/dtos";
import { useTransactions, useBaskets, useCommonData } from "#imports";
import UiTextInputUi from "../ui/TextInputUi.vue";
import TextAreaInput from "../ui/TextAreaInput.vue";

const emit = defineEmits<{
  success: [];
  cancel: [];
}>();

const { t } = useI18n();
const transactionService = useTransactions();
const basketService = useBaskets();
const commonService = useCommonData();

const formState = reactive<Partial<AllocatePayload>>({
  to_basket_id: undefined,
  amount: undefined,
  transaction_type_id: undefined,
  title: "",
  description: undefined,
});

const branchBaskets = ref<
  (Omit<BasketResponse, "description"> & { description?: string })[]
>([]);
const transactionTypes = ref<IdNameResponse[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function loadFormData() {
  try {
    const [basketsData, typesData] = await Promise.all([
      basketService.fetchBaskets({ limit: 100, page: 1 }),
      commonService.fetchTransactionTypes(),
    ]);

    branchBaskets.value = basketsData.data
      .filter((b) => b.basket_type !== "main")
      .map((b) => ({ ...b, description: b.description ?? undefined }));
    transactionTypes.value = typesData.data;
  } catch (e) {
    error.value = t("form.loadDataFailed");
  }
}

async function handleSubmit() {
  isLoading.value = true;
  error.value = null;

  try {
    await transactionService.allocate(formState as AllocatePayload);
    emit("success");
  } catch (e: any) {
    error.value =
      e.response?.data?.message || t("form.createTransactionFailed");
  } finally {
    isLoading.value = false;
  }
}

onMounted(() => {
  loadFormData();
});
</script>
