import { z } from 'zod'

export const CommonErrorResponseSchema = z.object({
  error: z.string(),
  code: z.number()
})

export type CommonErrorResponse = z.infer<typeof CommonErrorResponseSchema>

export const CommonResponseSchema = z.object({
  message: z.string(),
  code: z.number()
})

export type CommonResponse = z.infer<typeof CommonResponseSchema>

export const IdNameResponseSchema = z.object({
  id: z.number(),
  name: z.string()
})

export type IdNameResponse = z.infer<typeof IdNameResponseSchema>

export const createPaginatedResponseSchema = <T extends z.ZodTypeAny>(itemSchema: T) => z.object({
  data: z.array(itemSchema),
  page: z.number(),
  limit: z.number(),
  total: z.number()
})

export const PaginatedIdNameResponseSchema = createPaginatedResponseSchema(IdNameResponseSchema)

export type PaginatedIdNameResponse = z.infer<typeof PaginatedIdNameResponseSchema>

export const PaginationParamsSchema = z.object({
  page: z.number().optional(),
  limit: z.number().optional()
})

export type PaginationParams = z.infer<typeof PaginationParamsSchema>
