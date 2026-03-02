import { apiClient, API_AUTH_REFRESH } from '../../shared'
import type { RefreshTokenRequest, AuthResponse } from '../../shared'
import type { AxiosResponse, AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  try {
    const token = getCookie(event, 'auth_token') ?? ''

    if (!token) {
      throw createError({
        statusCode: 401,
        message: 'Unauthorized'
      })
    }

    const [_, tokenString] = token.split(' ')

    if (!tokenString) {
      throw createError({
        statusCode: 401,
        message: 'Invalid token format'
      })
    }

    const { data } = await apiClient.post<RefreshTokenRequest, AuthResponse>(
      API_AUTH_REFRESH,
      { token: tokenString }
    )

    // Update httpOnly cookie with new token
    setCookie(event, 'auth_token', `Bearer ${data.token}`, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      path: '/',
      maxAge: 60 * 60 * 24 * 7 // 7 days
    })

    return data.user
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      message: err.response?.data?.message || err.response?.data?.error || 'Refresh failed'
    })
  }
})
