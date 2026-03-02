import { useApiClient } from './api'
import type { 
  LoginPayload, 
  RegisterPayload, 
  AuthResponse
} from '~/dtos'

export const useAuthService = () => {
  const api = useApiClient()

  const login = async (payload: LoginPayload): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/auth/login', payload)
    api.setToken(response.token)
    return response
  }

  const register = async (payload: RegisterPayload): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/auth/register', payload)
    api.setToken(response.token)
    return response
  }

  const logout = () => {
    api.clearToken()
    navigateTo('/signin')
  }

  const isAuthenticated = (): boolean => {
    if (import.meta.client) {
      return !!localStorage.getItem('auth_token')
    }
    return false
  }

  return {
    login,
    register,
    logout,
    isAuthenticated
  }
}
