import { z } from 'zod'
import axios from 'axios'
import { LoginPayloadSchema } from '~/dtos'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const body = await readBody(event)
  
  // Validate request body
  const validation = LoginPayloadSchema.safeParse(body)
  if (!validation.success) {
    throw createError({
      statusCode: 400,
      message: validation.error.issues[0]?.message
    })
  }

  try {
    const response = await axios.post(
      `${config.public.apiBaseUrl}/auth/login`,
      validation.data
    )
    return response.data
  } catch (error: any) {
    throw createError({
      statusCode: error.response?.status || 500,
      message: error.response?.data?.error || 'Login failed'
    })
  }
})
