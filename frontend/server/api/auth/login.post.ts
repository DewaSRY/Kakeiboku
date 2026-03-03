import { apiClient, API_AUTH_LOGIN, LoginPayloadSchema } from '../../shared'
import type { LoginPayload, AuthResponse } from '../../shared'
import type { AxiosResponse, AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const body = await readBody<LoginPayload>(event)

  // Validate request body
  const validation = LoginPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      statusMessage: validation.error.issues[0]?.message
    })
  }

  try {
    const { data } = await apiClient.post<LoginPayload, AuthResponse>(
      API_AUTH_LOGIN,
      validation.data
    )

    // Set httpOnly cookie for secure token storage
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
      statusMessage: err.response?.data?.message || 'Login failed'
    })
  }
})
