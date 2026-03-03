// Re-export types from app dtos for server-side use
export type {
  // Auth types
  LoginPayload,
  RegisterPayload,
  UserProfile,
  AuthResponse,
  // Basket types
  BasketResponse,
  BasketCategoryResponse,
  CreateBasketPayload,
  UpdateBasketPayload,
  TransactionBasketInfo,
  // Transaction types
  TransactionTypeChildrenResponse,
  TransactionTypeInfo,
  TransactionTypeResponse,
  TransactionResponse,
  TransactionDetailResponse,
  TransactionWithDetails,
  CreateTransactionPayload,
  CreateTransactionTypePayload,
  // Common types
  CommonErrorResponse,
  CommonResponse,
  IdNameResponse,
  PaginatedIdNameResponse,
  PaginationParams
} from '~/dtos'

// Re-export schemas for validation
export {
  LoginPayloadSchema,
  RegisterPayloadSchema,
  CreateBasketPayloadSchema,
  UpdateBasketPayloadSchema,
  CreateTransactionPayloadSchema
} from '~/dtos'

// Server-specific types
export interface RefreshTokenRequest {
  token: string
}

export interface PaginatedResponse<T> {
  data: T[]
  page: number
  limit: number
  total: number
}
