// Auth API paths
export const API_AUTH_LOGIN = '/auth/login'
export const API_AUTH_REGISTER = '/auth/register'
export const API_AUTH_PROFILE = '/auth/profile'
export const API_AUTH_REFRESH = '/auth/refresh'

// Basket API paths
export const API_USER_BASKETS = '/user/baskets'
export const API_USER_BASKET_BY_ID = (id: string | number) => `/user/baskets/${id}`
export const API_USER_BASKET_TRANSACTIONS = (basketId: string | number) => `/user/baskets/${basketId}/transactions`

// Transaction API paths
export const API_USER_TRANSACTIONS = '/user/transactions'
export const API_USER_TRANSACTION_BY_ID = (id: string | number) => `/user/transactions/${id}`
export const API_USER_TRANSACTIONS_DEPOSIT = "/user/transactions/deposit";
export const API_USER_TRANSACTIONS_ALLOCATE = "/user/transactions/allocate";
export const API_USER_TRANSACTIONS_SPEND = "/user/transactions/spend";

// Common API paths
export const API_USER_COMMON_BASKET_CATEGORY = '/user/common/basket_category'
export const API_USER_COMMON_TRANSACTION_TYPE = '/user/common/transaction_type'

// Dashboard API paths
export const API_USER_DASHBOARD_MONEY_STASH = '/user/dashboard/money-stash'
export const API_USER_DASHBOARD_BRANCH_SUMMARY = '/user/dashboard/branch-summary'
