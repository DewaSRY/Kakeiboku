import type { UserProfile, LoginPayload, RegisterPayload } from '~/dtos'

export function useAuth() {
  const user = ref<UserProfile | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  async function login(payload: LoginPayload) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<UserProfile>('/api/auth/login', {
        method: 'POST',
        body: payload
      })

      user.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Login failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function register(payload: RegisterPayload) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<UserProfile>('/api/auth/register', {
        method: 'POST',
        body: payload
      })

      user.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Registration failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function logout() {
    isLoading.value = true
    error.value = null

    try {
      await $fetch('/api/auth/logout', {
        method: 'POST'
      })
      user.value = null
      navigateTo('/signin')
    } catch (err: any) {
      error.value = err?.data?.message || 'Logout failed'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function getProfile() {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<UserProfile>('/api/auth/profile', {
        method: 'GET'
      })

      user.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to get profile'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function refreshToken() {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<UserProfile>('/api/auth/refresh', {
        method: 'GET'
      })

      user.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to refresh token'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  return {
    user,
    isLoading,
    error,
    login,
    register,
    logout,
    getProfile,
    refreshToken
  }
}
