import { apiClient, API_USER_DASHBOARD_BRANCH_SUMMARY } from '../../shared'
import type { BranchSummaryResponse } from '../../shared'
import type { AxiosError } from 'axios'

export default defineEventHandler(async (event) => {
  const query = getQuery(event)
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized'
    })
  }

  try {
    const { data } = await apiClient.get<BranchSummaryResponse>(
      API_USER_DASHBOARD_BRANCH_SUMMARY,
      {
        // params: query,
        headers: { Authorization: token }
      }
    )
    return data
  } catch (error: any) {
    const err = error as AxiosError<any>
    throw createError({
      statusCode: err.response?.status || 500,
      statusMessage: err.response?.data?.message || 'Failed to fetch branch summary'
    })
  }
})
