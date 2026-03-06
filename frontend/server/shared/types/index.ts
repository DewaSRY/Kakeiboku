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
  DepositPayload,
  AllocatePayload,
  SpendPayload,
  // Common types
  CommonErrorResponse,
  CommonResponse,
  IdNameResponse,
  PaginatedIdNameResponse,
  PaginationParams,
  // Dashboard types
  UserMoneyStash,
  UserBranchPercent,
  UserMoneyStashResponse,
  BranchStats,
  BranchSummaryResponse,
  DateRangeQuery,
} from "~/dtos";

// Re-export schemas for validation
export {
  LoginPayloadSchema,
  RegisterPayloadSchema,
  CreateBasketPayloadSchema,
  UpdateBasketPayloadSchema,
  CreateTransactionPayloadSchema,
  DepositPayloadSchema,
  AllocatePayloadSchema,
  SpendPayloadSchema,
} from "~/dtos";

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
