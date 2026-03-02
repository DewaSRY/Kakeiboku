export default defineEventHandler(async (event) => {
  // Clear the auth cookie
  deleteCookie(event, 'auth_token', {
    path: '/'
  })

  return { message: 'Logged out successfully' }
})
