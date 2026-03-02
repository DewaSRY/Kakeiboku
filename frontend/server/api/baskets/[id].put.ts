import axios from 'axios'
import { UpdateBasketPayloadSchema } from '~/dtos'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const basketId = getRouterParam(event, 'id')
  const body = await readBody(event)
  const authHeader = getHeader(event, 'authorization')
  
  if (!authHeader) {
    throw createError({
      statusCode: 401,
      message: 'Unauthorized'
    })
  }

  // Validate request body
  const validation = UpdateBasketPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      message: validation.error.issues[0]?.message
    })
  }

  try {
    const response = await axios.put(
      `${config.public.apiBaseUrl}/user/baskets/${basketId}`,
      validation.data,
      { headers: { Authorization: authHeader } }
    )
    return response.data
  } catch (error: any) {
    throw createError({
      statusCode: error.response?.status || 500,
      message: error.response?.data?.error || 'Failed to update basket'
    })
  }
})
