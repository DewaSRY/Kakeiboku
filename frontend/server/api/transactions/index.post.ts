import axios from 'axios'
import { CreateTransactionPayloadSchema } from '~/dtos'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const body = await readBody(event)
  const authHeader = getHeader(event, 'authorization')
  
  if (!authHeader) {
    throw createError({
      statusCode: 401,
      message: 'Unauthorized'
    })
  }

  // Validate request body
  const validation = CreateTransactionPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      message: validation.error.issues[0]?.message
    })
  }

  try {
    const response = await axios.post(
      `${config.public.apiBaseUrl}/user/transactions`,
      validation.data,
      { headers: { Authorization: authHeader } }
    )
    return response.data
  } catch (error: any) {
    throw createError({
      statusCode: error.response?.status || 500,
      message: error.response?.data?.error || 'Failed to create transaction'
    })
  }
})
