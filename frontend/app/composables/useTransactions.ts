import type {
  TransactionResponse,
  TransactionWithDetails,
  CreateTransactionPayload,
  DepositPayload,
  AllocatePayload,
  SpendPayload,
  PaginationParams,
} from "~/dtos";

interface PaginatedTransactionResponse {
  data: TransactionResponse[];
  page: number;
  limit: number;
  total: number;
}

export function useTransactions() {
  const transactions = ref<PaginatedTransactionResponse | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  async function fetchTransactions(
    params: Partial<PaginationParams> = { limit: 20, page: 1 },
  ) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<PaginatedTransactionResponse>(
        "/api/transactions",
        {
          method: "GET",
          query: params,
        },
      );

      transactions.value = data;
      return data;
    } catch (err: any) {
      error.value = err?.data?.message || "Failed to fetch transactions";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchBasketTransactions(
    basketId: number,
    params: Partial<PaginationParams> = { limit: 20, page: 1 },
  ) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<PaginatedTransactionResponse>(
        `/api/baskets/${basketId}/transactions`,
        {
          method: "GET",
          query: params,
        },
      );

      transactions.value = data;
      return data;
    } catch (err: any) {
      error.value = err?.data?.message || "Failed to fetch basket transactions";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function createTransaction(payload: CreateTransactionPayload) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<TransactionResponse>("/api/transactions", {
        method: "POST",
        body: payload,
      });

      return data;
    } catch (err: any) {
      error.value = err?.data?.message || "Failed to create transaction";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function getTransactionDetails(transactionId: number) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<TransactionWithDetails>(
        `/api/transactions/${transactionId}`,
        { method: "GET" },
      );

      return data;
    } catch (err: any) {
      error.value = err?.data?.message || "Failed to fetch transaction details";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function getRecentTransactions(limit: number = 5) {
    const response = await fetchTransactions({ limit, page: 1 });
    return response?.data || [];
  }

  async function deposit(payload: DepositPayload) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<TransactionResponse>(
        "/api/transactions/deposit",
        {
          method: "POST",
          body: payload,
        },
      );

      return data;
    } catch (err: any) {
      error.value =
        err?.data?.message || "Failed to create deposit transaction";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function allocate(payload: AllocatePayload) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<TransactionResponse>(
        "/api/transactions/allocate",
        {
          method: "POST",
          body: payload,
        },
      );

      return data;
    } catch (err: any) {
      error.value =
        err?.data?.message || "Failed to create allocate transaction";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function spend(payload: SpendPayload) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await $fetch<TransactionResponse>(
        "/api/transactions/spend",
        {
          method: "POST",
          body: payload,
        },
      );

      return data;
    } catch (err: any) {
      error.value = err?.data?.message || "Failed to create spend transaction";
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  return {
    transactions,
    isLoading,
    error,
    fetchTransactions,
    fetchBasketTransactions,
    createTransaction,
    getTransactionDetails,
    getRecentTransactions,
    deposit,
    allocate,
    spend,
  };
}
