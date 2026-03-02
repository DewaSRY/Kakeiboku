import { apiClient, API_USER_COMMON_BASKET_CATEGORY } from '../../shared'
import type { PaginatedIdNameResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const query = getQuery(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      message: 'Unauthorized'
    })
  }

  try {
    const { data } = await apiClient.get<PaginatedIdNameResponse>(
      API_USER_COMMON_BASKET_CATEGORY,
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
      message: err.response?.data?.message || err.response?.data?.error || 'Failed to fetch basket categories'
    })
  }
})
