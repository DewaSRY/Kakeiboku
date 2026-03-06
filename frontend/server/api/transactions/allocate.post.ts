import { apiClient, API_USER_TRANSACTIONS_ALLOCATE, AllocatePayloadSchema } from '../../shared'
import type { AllocatePayload, TransactionResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const body = await readBody<AllocatePayload>(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  const validation = AllocatePayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      statusMessage: validation.error.issues[0]?.message
    })
  }

  try {
    const { data } = await apiClient.post<AllocatePayload, TransactionResponse>(
      API_USER_TRANSACTIONS_ALLOCATE,
      validation.data,
      { headers: { Authorization: token } }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.error || 'Failed to allocate funds'
    })
  }
})
