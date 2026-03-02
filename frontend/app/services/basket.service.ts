import { useApiClient } from './api'
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

export const useBasketService = () => {
  const api = useApiClient()

  const getAllBaskets = async (params?: PaginationParams): Promise<PaginatedBasketResponse> => {
    return await api.get<PaginatedBasketResponse>('/user/baskets', { params })
  }

  const createBasket = async (payload: CreateBasketPayload): Promise<CommonResponse> => {
    return await api.post<CommonResponse>('/user/baskets', payload)
  }

  const updateBasket = async (basketId: number, payload: UpdateBasketPayload): Promise<CommonResponse> => {
    return await api.put<CommonResponse>(`/user/baskets/${basketId}`, payload)
  }

  const getMainBasket = async (): Promise<BasketResponse | null> => {
    const baskets = await getAllBaskets()
    return baskets.data.find(b => b.basket_type === 'main') || null
  }

  const getBranchBaskets = async (): Promise<BasketResponse[]> => {
    const baskets = await getAllBaskets()
    return baskets.data.filter(b => b.basket_type !== 'main')
  }

  const getTotalBalance = async (): Promise<number> => {
    const baskets = await getAllBaskets()
    return baskets.data.reduce((acc, basket) => acc + basket.balance, 0)
  }

  return {
    getAllBaskets,
    createBasket,
    updateBasket,
    getMainBasket,
    getBranchBaskets,
    getTotalBalance
  }
}
