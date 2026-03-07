<template>
  <FormContainer :title="$t('baskets.createNewBasket')">
    <UForm :schema="CreateBasketPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UiTextInputUi
        v-model="formState.name"
        :label="$t('baskets.basketName')"
        name="name"
        :placeholder="$t('baskets.basketNamePlaceholder')"
        icon="i-lucide-archive"
      />

      <UiSelectOption
        v-model="formState.basket_category_id"
        :items="categories"
        :label="$t('common.category')"
        name="basket_category_id"
        :placeholder="$t('baskets.selectCategory')"
        required
      />

      <TextAreaInput
        v-model="formState.description"
        :label="$t('common.description')"
        name="description"
        :placeholder="$t('baskets.optionalDescription')"
        :rows="3"
      />

      <FormActions 
        :submit-text="$t('baskets.createBasket')" 
        :loading="isLoading"
        @cancel="$emit('cancel')"
      />
    </UForm>

    <FormError :error="error" />
  </FormContainer>
</template>

<script setup lang="ts">
import { CreateBasketPayloadSchema, type CreateBasketPayload, type IdNameResponse } from '~/dtos'
import { useBaskets, useCommonData } from '#imports';
import TextAreaInput from '../ui/TextAreaInput.vue';
const emit = defineEmits<{
  success: []
  cancel: []
}>()

const { t } = useI18n()
const basketService = useBaskets()
const commonService = useCommonData()

const formState = reactive<Partial<CreateBasketPayload>>({
  name: '',
  basket_category_id: undefined,
  description: undefined
})

const categories = ref<IdNameResponse[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

async function loadCategories() {
  try {
    const response = await commonService.fetchBasketCategories()
    categories.value = response.data
  } catch (e) {
    error.value = t('baskets.loadCategoriesFailed')
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await basketService.createBasket(formState as CreateBasketPayload)
    emit('success')
  } catch (e: any) {
    error.value = e.response?.data?.message || t('baskets.createFailed')
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadCategories()
})
</script>
