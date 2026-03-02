import { z } from 'zod'

// Common Error Response
export const CommonErrorResponseSchema = z.object({
  error: z.string(),
  code: z.number()
})

export type CommonErrorResponse = z.infer<typeof CommonErrorResponseSchema>

// Common Response
export const CommonResponseSchema = z.object({
  message: z.string(),
  code: z.number()
})

export type CommonResponse = z.infer<typeof CommonResponseSchema>

// Id Name Response
export const IdNameResponseSchema = z.object({
  id: z.number(),
  name: z.string()
})

export type IdNameResponse = z.infer<typeof IdNameResponseSchema>

// Paginated Response Generic
export const createPaginatedResponseSchema = <T extends z.ZodTypeAny>(itemSchema: T) => z.object({
  data: z.array(itemSchema),
  page: z.number(),
  limit: z.number(),
  total: z.number()
})

// Paginated Response for Id Name
export const PaginatedIdNameResponseSchema = createPaginatedResponseSchema(IdNameResponseSchema)

export type PaginatedIdNameResponse = z.infer<typeof PaginatedIdNameResponseSchema>

// Pagination Params
export const PaginationParamsSchema = z.object({
  page: z.number().optional(),
  limit: z.number().optional()
})

export type PaginationParams = z.infer<typeof PaginationParamsSchema>
