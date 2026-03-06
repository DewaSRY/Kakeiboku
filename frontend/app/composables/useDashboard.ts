import type { 
  UserMoneyStashResponse, 
  BranchSummaryResponse,
  DateRangeQuery
} from '~/dtos'

export function useDashboard() {
  const moneyStash = ref<UserMoneyStashResponse | null>(null)
  const branchSummary = ref<BranchSummaryResponse | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  async function fetchMoneyStash() {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<UserMoneyStashResponse>('/api/dashboard/money-stash', {
        method: 'GET'
      })

      moneyStash.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to fetch money stash'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function fetchBranchSummary(params?: DateRangeQuery) {
    isLoading.value = true
    error.value = null

    try {
      const data = await $fetch<BranchSummaryResponse>('/api/dashboard/branch-summary', {
        method: 'GET',
        query: params
      })

      branchSummary.value = data
      return data
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to fetch branch summary'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function loadDashboardStats(dateRange?: DateRangeQuery) {
    isLoading.value = true
    error.value = null

    try {
      const [stash, summary] = await Promise.all([
        fetchMoneyStash(),
        fetchBranchSummary(dateRange)
      ])

      return { moneyStash: stash, branchSummary: summary }
    } catch (err: any) {
      error.value = err?.data?.message || 'Failed to load dashboard data'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  return {
    moneyStash,
    branchSummary,
    isLoading,
    error,
    fetchMoneyStash,
    fetchBranchSummary,
    loadDashboardStats
  }
}
