import { apiClient, API_USER_BASKETS, CreateBasketPayloadSchema } from '../../shared'
import type { CreateBasketPayload, CommonResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const body = await readBody<CreateBasketPayload>(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  // Validate request body
  const validation = CreateBasketPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      statusMessage: validation.error.issues[0]?.message
    })
  }

  try {
    const { data } = await apiClient.post<CreateBasketPayload, CommonResponse>(
      API_USER_BASKETS,
      validation.data,
      { headers: { Authorization: token } }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.message || 'Failed to create basket'
    })
  }
})
