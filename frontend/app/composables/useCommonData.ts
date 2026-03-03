import type { PaginatedIdNameResponse, PaginationParams } from '~/dtos'

export function useCommonData() {
  const basketCategories = ref<PaginatedIdNameResponse | null>(null)
  const transactionTypes = ref<PaginatedIdNameResponse | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  async function fetchBasketCategories(params: Partial<PaginationParams> = { limit: 100, page: 1 }) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<PaginatedIdNameResponse>('/api/common/basket-categories', {
        method: 'GET',
        query: params
      })

      basketCategories.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to fetch basket categories'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function fetchTransactionTypes(params: Partial<PaginationParams> = { limit: 100, page: 1 }) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<PaginatedIdNameResponse>('/api/common/transaction-types', {
        method: 'GET',
        query: params
      })

      transactionTypes.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to fetch transaction types'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  return {
    basketCategories,
    transactionTypes,
    isLoading,
    error,
    fetchBasketCategories,
    fetchTransactionTypes
  }
}
