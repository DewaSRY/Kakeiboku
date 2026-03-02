<template>
  <FormContainer :title="$t('baskets.createNewBasket')">
    <UForm :schema="CreateBasketPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UFormField :label="$t('baskets.basketName')" name="name">
        <UInput 
          v-model="formState.name" 
          :placeholder="$t('baskets.basketNamePlaceholder')"
          size="lg"
        />
      </UFormField>

      <UFormField :label="$t('common.category')" name="basket_category_id">
        <USelect
          v-model="formState.basket_category_id"
          :items="categories"
          value-key="id"
          label-key="name"
          :placeholder="$t('baskets.selectCategory')"
          size="lg"
        />
      </UFormField>

      <UFormField :label="$t('common.description')" name="description">
        <UTextarea 
          v-model="formState.description" 
          :placeholder="$t('baskets.optionalDescription')"
          :rows="3"
        />
      </UFormField>

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
import { useBasketService, useCommonService } from '~/services'

const emit = defineEmits<{
  success: []
  cancel: []
}>()

const { t } = useI18n()
const basketService = useBasketService()
const commonService = useCommonService()

const formState = reactive<CreateBasketPayload>({
  name: '',
  basket_category_id: 0,
  description: null
})

const categories = ref<IdNameResponse[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

async function loadCategories() {
  try {
    const response = await commonService.getBasketCategories()
    categories.value = response.data
  } catch (e) {
    error.value = t('baskets.loadCategoriesFailed')
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await basketService.createBasket(formState)
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
