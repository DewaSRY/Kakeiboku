import { apiClient, API_AUTH_PROFILE } from '../../shared'
import type { UserProfile } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  try {
    const token = getCookie(event, 'auth_token') ?? ''

    if (!token) {
      throw createError({
        statusCode: 401,
        message: 'Unauthorized'
      })
    }

    const { data } = await apiClient.get<UserProfile>(API_AUTH_PROFILE, {
      headers: {
        Authorization: token
      }
    })

    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      message: err.response?.data?.message || err.response?.data?.error || 'Get profile failed'
    })
  }
})
