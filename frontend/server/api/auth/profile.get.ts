import { apiClient, API_AUTH_PROFILE } from '../../shared'
import type { UserProfile } from '../../shared'
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
      statusMessage: err.response?.data?.message || 'Get profile failed'
    })
  }
})
