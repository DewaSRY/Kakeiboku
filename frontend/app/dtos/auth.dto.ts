import { z } from 'zod'

export const LoginPayloadSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(1, 'Password is required')
})

export type LoginPayload = z.infer<typeof LoginPayloadSchema>

export const RegisterPayloadSchema = z.object({
  first_name: z.string().min(1, 'First name is required'),
  last_name: z.string().min(1, 'Last name is required'),
  email: z.string().email('Invalid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters')
})

export type RegisterPayload = z.infer<typeof RegisterPayloadSchema>

export const UserProfileSchema = z.object({
  id: z.number(),
  first_name: z.string(),
  last_name: z.string(),
  email: z.string()
})

export type UserProfile = z.infer<typeof UserProfileSchema>

export const AuthResponseSchema = z.object({
  token: z.string(),
  user: UserProfileSchema
})

export type AuthResponse = z.infer<typeof AuthResponseSchema>
