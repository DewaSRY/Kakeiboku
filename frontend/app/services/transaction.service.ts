import { useApiClient } from './api'
import type { 
  TransactionResponse,
  TransactionWithDetails,
  CreateTransactionPayload,
  PaginationParams
} from '~/dtos'

interface PaginatedTransactionResponse {
  data: TransactionResponse[]
  page: number
  limit: number
  total: number
}

interface PaginatedTransactionWithDetailsResponse {
  data: TransactionWithDetails[]
  page: number
  limit: number
  total: number
}

export const useTransactionService = () => {
  const api = useApiClient()

  const getBasketTransactions = async (
    basketId: number, 
    params?: PaginationParams
  ): Promise<PaginatedTransactionResponse> => {
    return await api.get<PaginatedTransactionResponse>(`/user/baskets/${basketId}/transactions`, { params })
  }

  const createTransaction = async (payload: CreateTransactionPayload): Promise<TransactionResponse> => {
    return await api.post<TransactionResponse>('/user/transactions', payload)
  }

  const getAllUserTransactions = async (params?: PaginationParams): Promise<PaginatedTransactionResponse> => {
    return await api.get<PaginatedTransactionResponse>('/user/transactions', { params })
  }

  const getTransactionDetails = async (transactionId: number): Promise<TransactionWithDetails> => {
    return await api.get<TransactionWithDetails>(`/user/transactions/${transactionId}`)
  }

  const getRecentTransactions = async (limit: number = 5): Promise<TransactionResponse[]> => {
    const response = await getAllUserTransactions({ limit, page: 1 })
    return response.data
  }

  return {
    getBasketTransactions,
    createTransaction,
    getAllUserTransactions,
    getTransactionDetails,
    getRecentTransactions
  }
}
