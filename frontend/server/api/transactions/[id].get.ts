import { apiClient, API_USER_TRANSACTION_BY_ID } from '../../shared'
import type { TransactionWithDetails } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const transactionId = getRouterParam(event, 'id')
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      message: 'Unauthorized'
    })
  }

  try {
    const { data } = await apiClient.get<TransactionWithDetails>(
      API_USER_TRANSACTION_BY_ID(transactionId!),
      { headers: { Authorization: token } }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      message: err.response?.data?.message || err.response?.data?.error || 'Failed to fetch transaction'
    })
  }
})
