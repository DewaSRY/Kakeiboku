export default defineNuxtRouteMiddleware((to, from) => {
  if (import.meta.client) {
    const token = localStorage.getItem('auth_token')
    if (!token && to.path !== '/signin' && to.path !== '/signup') {
      return navigateTo('/signin')
    }
  }
})
