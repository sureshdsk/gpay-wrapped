import { api } from './client'
import type { Feature, UserFeature } from '@/types'

export const featuresApi = {
  listAll: () => api.get<Feature[]>('/v1/features'),

  getUserFeatures: () => api.get<UserFeature[]>('/v1/user/features'),

  enableFeature: (featureId: number) =>
    api.post<{ status: string }>(`/v1/user/features/${featureId}/enable`),

  disableFeature: (featureId: number) =>
    api.post<{ status: string }>(`/v1/user/features/${featureId}/disable`),

  toggleFeature: (featureId: number, enabled: boolean) =>
    api.put<{ status: string }>(`/v1/user/features/${featureId}`, { enabled }),

  checkFeature: (featureKey: string) =>
    api.get<{ feature: string; enabled: boolean }>(`/v1/user/features/check/${featureKey}`),
}
