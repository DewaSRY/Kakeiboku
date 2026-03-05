import { apiClient, API_USER_DASHBOARD_MONEY_STASH } from '../../shared'
import type { UserMoneyStashResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  try {
    const { data } = await apiClient.get<UserMoneyStashResponse>(
      API_USER_DASHBOARD_MONEY_STASH,
      {
        headers: { Authorization: token }
      }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.message || 'Failed to fetch money stash'
    })
  }
})
