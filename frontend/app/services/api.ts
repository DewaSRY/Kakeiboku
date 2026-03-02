import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosError } from 'axios'
import type { CommonErrorResponse } from '~/dtos'

class ApiClient {
  private instance: AxiosInstance
  private token: string | null = null

  constructor() {
    const config = useRuntimeConfig()
    
    this.instance = axios.create({
      baseURL: config.public.apiBaseUrl as string,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    })

    // Request interceptor for auth token
    this.instance.interceptors.request.use(
      (config) => {
        if (this.token) {
          config.headers.Authorization = `Bearer ${this.token}`
        }
        return config
      },
      (error) => Promise.reject(error)
    )

    // Response interceptor for error handling
    this.instance.interceptors.response.use(
      (response) => response,
      (error: AxiosError<CommonErrorResponse>) => {
        if (error.response?.status === 401) {
          this.clearToken()
          // Redirect to login if unauthorized
          if (import.meta.client) {
            navigateTo('/signin')
          }
        }
        return Promise.reject(error)
      }
    )
  }

  setToken(token: string) {
    this.token = token
    if (import.meta.client) {
      localStorage.setItem('auth_token', token)
    }
  }

  clearToken() {
    this.token = null
    if (import.meta.client) {
      localStorage.removeItem('auth_token')
    }
  }

  loadToken() {
    if (import.meta.client) {
      const token = localStorage.getItem('auth_token')
      if (token) {
        this.token = token
      }
    }
  }

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.instance.get<T>(url, config)
    return response.data
  }

  async post<T>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.instance.post<T>(url, data, config)
    return response.data
  }

  async put<T>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.instance.put<T>(url, data, config)
    return response.data
  }

  async delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.instance.delete<T>(url, config)
    return response.data
  }
}

// Singleton instance
let apiClient: ApiClient | null = null

export const useApiClient = () => {
  if (!apiClient) {
    apiClient = new ApiClient()
    apiClient.loadToken()
  }
  return apiClient
}

export default ApiClient
