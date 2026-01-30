import { api } from './client'
import type { DashboardSummary } from '@/types'

export const insightsApi = {
  getDashboardSummary: () =>
    api.get<DashboardSummary>('/v1/insights/summary'),
}
