import { z } from 'zod'

export const BasketResponseSchema = z.object({
  id: z.number(),
  user_id: z.number(),
  name: z.string(),
  description: z.string().nullable().optional(),
  basket_category_id: z.number(),
  basket_type: z.string(),
  status: z.string(),
  balance: z.number(),
  created_at: z.string(),
  updated_at: z.string()
})

export type BasketResponse = z.infer<typeof BasketResponseSchema>

export const BasketCategoryResponseSchema = z.object({
  id: z.number(),
  name: z.string(),
  description: z.string().nullable().optional(),
  created_at: z.string()
})

export type BasketCategoryResponse = z.infer<typeof BasketCategoryResponseSchema>

export const CreateBasketPayloadSchema = z.object({
  name: z.string().min(1, 'Basket name is required'),
  basket_category_id: z.number().min(1, 'Category is required'),
  description: z.string().nullable().optional()
})

export type CreateBasketPayload = z.infer<typeof CreateBasketPayloadSchema>

export const UpdateBasketPayloadSchema = z.object({
  name: z.string().nullable().optional(),
  description: z.string().nullable().optional(),
  status: z.string().nullable().optional()
})

export type UpdateBasketPayload = z.infer<typeof UpdateBasketPayloadSchema>

export const TransactionBasketInfoSchema = z.object({
  id: z.number(),
  name: z.string(),
  basket_type: z.string()
})

export type TransactionBasketInfo = z.infer<typeof TransactionBasketInfoSchema>
