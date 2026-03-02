import { useApiClient } from './api'
import type { 
  PaginatedIdNameResponse,
  PaginationParams
} from '~/dtos'

export const useCommonService = () => {
  const api = useApiClient()

  const getBasketCategories = async (params?: PaginationParams): Promise<PaginatedIdNameResponse> => {
    return await api.get<PaginatedIdNameResponse>('/user/common/basket_category', { params })
  }

  const getTransactionTypes = async (params?: PaginationParams): Promise<PaginatedIdNameResponse> => {
    return await api.get<PaginatedIdNameResponse>('/user/common/transaction_type', { params })
  }

  return {
    getBasketCategories,
    getTransactionTypes
  }
}
