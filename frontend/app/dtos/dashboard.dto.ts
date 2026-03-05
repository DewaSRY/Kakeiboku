import { z } from 'zod'

// Money Stash DTOs
export const UserMoneyStashSchema = z.object({
  main_branch: z.number(),
  total_spend: z.number(),
  total_save: z.number()
})

export type UserMoneyStash = z.infer<typeof UserMoneyStashSchema>

export const UserBranchPercentSchema = z.object({
  branch_category_name: z.string(),
  total_percent: z.number()
})

export type UserBranchPercent = z.infer<typeof UserBranchPercentSchema>

export const UserMoneyStashResponseSchema = z.object({
  money_stash: UserMoneyStashSchema,
  branch_category_percentages: z.array(UserBranchPercentSchema)
})

export type UserMoneyStashResponse = z.infer<typeof UserMoneyStashResponseSchema>

// Branch Summary DTOs
export const BranchStatsSchema = z.object({
  name: z.string(),
  total: z.number()
})

export type BranchStats = z.infer<typeof BranchStatsSchema>

export const BranchSummaryResponseSchema = z.object({
  data: z.array(BranchStatsSchema)
})

export type BranchSummaryResponse = z.infer<typeof BranchSummaryResponseSchema>

// Date Range Query for branch summary
export const DateRangeQuerySchema = z.object({
  start_date: z.string().optional(),
  end_date: z.string().optional()
})

export type DateRangeQuery = z.infer<typeof DateRangeQuerySchema>
