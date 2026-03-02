import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosResponse, type AxiosError } from 'axios'

class ServerApiClient {
  private instance: AxiosInstance

  constructor() {
    const config = useRuntimeConfig()

    this.instance = axios.create({
      baseURL: config.public.apiBaseUrl as string,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    })
  }

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
    return this.instance.get<T>(url, config)
  }

  async post<TRequest, TResponse>(
    url: string,
    data?: TRequest,
    config?: AxiosRequestConfig
  ): Promise<AxiosResponse<TResponse>> {
    return this.instance.post<TResponse>(url, data, config)
  }

  async put<TRequest, TResponse>(
    url: string,
    data?: TRequest,
    config?: AxiosRequestConfig
  ): Promise<AxiosResponse<TResponse>> {
    return this.instance.put<TResponse>(url, data, config)
  }

  async delete<T>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {
    return this.instance.delete<T>(url, config)
  }
}

// Create and export singleton instance
export const apiClient = new ServerApiClient()
