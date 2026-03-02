import type { 
  BasketResponse, 
  CreateBasketPayload, 
  UpdateBasketPayload, 
  CommonResponse,
  PaginationParams 
} from '~/dtos'

interface PaginatedBasketResponse {
  data: BasketResponse[]
  page: number
  limit: number
  total: number
}

export function useBaskets() {
  const baskets = ref<PaginatedBasketResponse | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  async function fetchBaskets(params: Partial<PaginationParams> = { limit: 20, page: 1 }) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<PaginatedBasketResponse>('/api/baskets', {
        method: 'GET',
        query: params
      })

      baskets.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to fetch baskets'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function createBasket(payload: CreateBasketPayload) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<CommonResponse>('/api/baskets', {
        method: 'POST',
        body: payload
      })

      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to create basket'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function updateBasket(basketId: number, payload: UpdateBasketPayload) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<CommonResponse>(`/api/baskets/${basketId}`, {
        method: 'PUT',
        body: payload
      })

      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to update basket'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const getMainBasket = computed(() => {
    return baskets.value?.data.find(b => b.basket_type === 'main') || null
  })

  const getBranchBaskets = computed(() => {
    return baskets.value?.data.filter(b => b.basket_type !== 'main') || []
  })

  const getTotalBalance = computed(() => {
    return baskets.value?.data.reduce((acc, basket) => acc + basket.balance, 0) || 0
  })

  return {
    baskets,
    isLoading,
    error,
    fetchBaskets,
    createBasket,
    updateBasket,
    getMainBasket,
    getBranchBaskets,
    getTotalBalance
  }
}
