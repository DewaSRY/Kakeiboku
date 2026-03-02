import { apiClient, API_USER_BASKET_TRANSACTIONS } from '../../../shared'
import type { TransactionResponse, PaginatedResponse } from '../../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const basketId = getRouterParam(event, 'basketId')
  const query = getQuery(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  try {
    const { data } = await apiClient.get<PaginatedResponse<TransactionResponse>>(
      API_USER_BASKET_TRANSACTIONS(basketId!),
      {
        params: query,
        headers: { Authorization: token }
      }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.message || 'Failed to fetch transactions'
    })
  }
})
