import axios from 'axios'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const basketId = getRouterParam(event, 'basketId')
  const query = getQuery(event)
  const authHeader = getHeader(event, 'authorization')
  
  if (!authHeader) {
    throw createError({
      statusCode: 401,
      message: 'Unauthorized'
    })
  }

  try {
    const response = await axios.get(
      `${config.public.apiBaseUrl}/user/baskets/${basketId}/transactions`,
      {
        params: query,
        headers: { Authorization: authHeader }
      }
    )
    return response.data
  } catch (error: any) {
    throw createError({
      statusCode: error.response?.status || 500,
      message: error.response?.data?.error || 'Failed to fetch transactions'
    })
  }
})
