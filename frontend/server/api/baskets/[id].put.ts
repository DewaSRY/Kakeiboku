import { apiClient, API_USER_BASKET_BY_ID, UpdateBasketPayloadSchema } from '../../shared'
import type { UpdateBasketPayload, CommonResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const basketId = getRouterParam(event, 'id')
  const body = await readBody<UpdateBasketPayload>(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  // Validate request body
  const validation = UpdateBasketPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      statusMessage: validation.error.issues[0]?.message
    })
  }

  try {
    const { data } = await apiClient.put<UpdateBasketPayload, CommonResponse>(
      API_USER_BASKET_BY_ID(basketId!),
      validation.data,
      { headers: { Authorization: token } }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.message || 'Failed to update basket'
    })
  }
})
