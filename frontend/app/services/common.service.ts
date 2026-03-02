import { useApiClient } from './api'
import type { 
  PaginatedIdNameResponse,
  PaginationParams
} from '~/dtos'

export const useCommonService = () => {
  const api = useApiClient()

  const getBasketCategories = async (params?: PaginationParams): Promise<PaginatedIdNameResponse> => {
    return await api.get<PaginatedIdNameResponse>('/api/common/basket-categories', params)
  }

  const getTransactionTypes = async (params?: PaginationParams): Promise<PaginatedIdNameResponse> => {
    return await api.get<PaginatedIdNameResponse>('/api/common/transaction-types', params)
  }

  return {
    getBasketCategories,
    getTransactionTypes
  }
}
