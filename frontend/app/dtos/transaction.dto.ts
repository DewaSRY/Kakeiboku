import { z } from 'zod'
import { TransactionBasketInfoSchema } from './basket.dto'

export const TransactionTypeChildrenResponseSchema = z.object({
  id: z.number(),
  name: z.string(),
  description: z.string().nullable().optional(),
  parent_id: z.number().nullable().optional(),
  created_at: z.string()
})

export type TransactionTypeChildrenResponse = z.infer<typeof TransactionTypeChildrenResponseSchema>

export const TransactionTypeInfoSchema = z.object({
  id: z.number(),
  name: z.string(),
  parent_id: z.number().nullable().optional()
})

export type TransactionTypeInfo = z.infer<typeof TransactionTypeInfoSchema>

export const TransactionTypeResponseSchema = z.object({
  id: z.number(),
  name: z.string(),
  description: z.string().nullable().optional(),
  parent_id: z.number().nullable().optional(),
  children: z.array(TransactionTypeChildrenResponseSchema),
  created_at: z.string()
})

export type TransactionTypeResponse = z.infer<typeof TransactionTypeResponseSchema>

export const TransactionResponseSchema = z.object({
  id: z.number(),
  created_by_id: z.number(),
  from_basket_id: z.number().nullable().optional(),
  to_basket_id: z.number(),
  amount: z.number(),
  transaction_type_id: z.number(),
  created_at: z.string()
})

export type TransactionResponse = z.infer<typeof TransactionResponseSchema>

export const TransactionDetailResponseSchema = z.object({
  id: z.number(),
  transaction_id: z.number(),
  title: z.string(),
  description: z.string().nullable().optional(),
  created_at: z.string()
})

export type TransactionDetailResponse = z.infer<typeof TransactionDetailResponseSchema>

// Transaction With Details
export const TransactionWithDetailsSchema = z.object({
  id: z.number(),
  created_by_id: z.number(),
  from_basket: TransactionBasketInfoSchema,
  to_basket: TransactionBasketInfoSchema,
  amount: z.number(),
  transaction_type: TransactionTypeInfoSchema,
  detail: TransactionDetailResponseSchema.nullable().optional(),
  created_at: z.string()
})

export type TransactionWithDetails = z.infer<typeof TransactionWithDetailsSchema>

// Create Transaction Payload
export const CreateTransactionPayloadSchema = z.object({
  from_basket_id: z.number(),
  to_basket_id: z.number(),
  amount: z.number().min(0.01, 'Amount must be greater than 0'),
  transaction_type_id: z.number(),
  title: z.string().min(1, 'Title is required'),
  description: z.string().nullable().optional()
})

export type CreateTransactionPayload = z.infer<typeof CreateTransactionPayloadSchema>

// Create Transaction Type Payload
export const CreateTransactionTypePayloadSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  description: z.string().nullable().optional(),
  parent_id: z.number().nullable().optional()
})

export type CreateTransactionTypePayload = z.infer<typeof CreateTransactionTypePayloadSchema>

// Update Transaction Type Payload
export const UpdateTransactionTypePayloadSchema = z.object({
  name: z.string().nullable().optional(),
  description: z.string().nullable().optional(),
  parent_id: z.number().nullable().optional()
})

export type UpdateTransactionTypePayload = z.infer<typeof UpdateTransactionTypePayloadSchema>
