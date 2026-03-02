<template>
  <div class="p-6">
    <h2 class="text-xl font-bold text-gray-900 mb-6">Create New Basket</h2>
    
    <UForm :schema="CreateBasketPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UFormField label="Basket Name" name="name">
        <UInput 
          v-model="formState.name" 
          placeholder="e.g., Food, Savings, Entertainment"
          size="lg"
        />
      </UFormField>

      <UFormField label="Category" name="basket_category_id">
        <USelect
          v-model="formState.basket_category_id"
          :items="categories"
          value-key="id"
          label-key="name"
          placeholder="Select a category"
          size="lg"
        />
      </UFormField>

      <UFormField label="Description" name="description">
        <UTextarea 
          v-model="formState.description" 
          placeholder="Optional description..."
          :rows="3"
        />
      </UFormField>

      <div class="flex justify-end space-x-3 pt-4">
        <UButton 
          color="neutral" 
          variant="ghost" 
          @click="$emit('cancel')"
        >
          Cancel
        </UButton>
        <UButton 
          type="submit" 
          color="primary" 
          :loading="isLoading"
        >
          Create Basket
        </UButton>
      </div>
    </UForm>

    <UAlert 
      v-if="error" 
      color="error" 
      variant="soft" 
      :title="error"
      class="mt-4"
    />
  </div>
</template>

<script setup lang="ts">
import { CreateBasketPayloadSchema, type CreateBasketPayload, type IdNameResponse } from '~/dtos'
import { useBasketService, useCommonService } from '~/services'

const emit = defineEmits<{
  success: []
  cancel: []
}>()

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
    error.value = 'Failed to load categories'
  }
}

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await basketService.createBasket(formState)
    emit('success')
  } catch (e: any) {
    error.value = e.response?.data?.message || 'Failed to create basket'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadCategories()
})
</script>
