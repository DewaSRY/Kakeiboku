import { useApiClient } from './api'
import type { 
  LoginPayload, 
  RegisterPayload, 
  UserProfile
} from '~/dtos'

export const useAuthService = () => {
  const api = useApiClient()

  const login = async (payload: LoginPayload): Promise<UserProfile> => {
    return await api.post<UserProfile>('/api/auth/login', payload)
  }

  const register = async (payload: RegisterPayload): Promise<UserProfile> => {
    return await api.post<UserProfile>('/api/auth/register', payload)
  }

  const logout = async () => {
    await api.post('/api/auth/logout')
    navigateTo('/signin')
  }

  const getProfile = async (): Promise<UserProfile> => {
    return await api.get<UserProfile>('/api/auth/profile')
  }

  const refreshToken = async (): Promise<UserProfile> => {
    return await api.get<UserProfile>('/api/auth/refresh')
  }

  return {
    login,
    register,
    logout,
    getProfile,
    refreshToken
  }
}
