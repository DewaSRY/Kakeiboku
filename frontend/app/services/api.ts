import type { CommonErrorResponse } from '~/dtos'

/**
 * Internal API client for calling Nuxt server endpoints.
 * All endpoints use httpOnly cookies for authentication, so no token management is needed.
 */
class ApiClient {
  async get<T>(url: string, params?: Record<string, any>): Promise<T> {
    return await $fetch<T>(url, {
      method: 'GET',
      params,
      credentials: 'include'
    })
  }

  async post<T>(url: string, data?: unknown): Promise<T> {
    return await $fetch<T>(url, {
      method: 'POST',
      body: data,
      credentials: 'include'
    })
  }

  async put<T>(url: string, data?: unknown): Promise<T> {
    return await $fetch<T>(url, {
      method: 'PUT',
      body: data,
      credentials: 'include'
    })
  }

  async delete<T>(url: string): Promise<T> {
    return await $fetch<T>(url, {
      method: 'DELETE',
      credentials: 'include'
    })
  }
}

// Singleton instance
let apiClient: ApiClient | null = null

export const useApiClient = () => {
  if (!apiClient) {
    apiClient = new ApiClient()
  }
  return apiClient
}

export default ApiClient
