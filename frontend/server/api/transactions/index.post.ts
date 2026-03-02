import { apiClient, API_USER_TRANSACTIONS, CreateTransactionPayloadSchema } from '../../shared'
import type { CreateTransactionPayload, TransactionResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const body = await readBody<CreateTransactionPayload>(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  // Validate request body
  const validation = CreateTransactionPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      statusMessage: validation.error.issues[0]?.message
    })
  }

  try {
    const { data } = await apiClient.post<CreateTransactionPayload, TransactionResponse>(
      API_USER_TRANSACTIONS,
      validation.data,
      { headers: { Authorization: token } }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.message || 'Failed to create transaction'
    })
  }
})
