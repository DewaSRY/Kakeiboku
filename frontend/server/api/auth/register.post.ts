import { apiClient, API_AUTH_REGISTER, RegisterPayloadSchema } from '../../shared'
import type { RegisterPayload, AuthResponse } from '../../shared'
import type { AxiosResponse, AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const body = await readBody<RegisterPayload>(event)

  // Validate request body
  const validation = RegisterPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      statusMessage: validation.error.issues[0]?.message
    })
  }

  try {
    const { data } = await apiClient.post<RegisterPayload, AuthResponse>(
      API_AUTH_REGISTER,
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
      statusMessage: err.response?.data?.message || 'Registration failed'
    })
  }
})
