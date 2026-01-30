import { create } from 'zustand'
import { featuresApi } from '@/lib/api/features'
import type { UserFeature } from '@/types'

interface FeatureState {
  features: UserFeature[]
  isLoading: boolean
  error: string | null

  fetchFeatures: () => Promise<void>
  toggleFeature: (featureId: number, enabled: boolean) => Promise<void>
  isFeatureEnabled: (featureKey: string) => boolean
}

export const useFeatureStore = create<FeatureState>()((set, get) => ({
  features: [],
  isLoading: false,
  error: null,

  fetchFeatures: async () => {
    set({ isLoading: true, error: null })
    try {
      const features = await featuresApi.getUserFeatures()
      set({ features, isLoading: false })
    } catch (error) {
      set({
        isLoading: false,
        error: error instanceof Error ? error.message : 'Failed to fetch features',
      })
    }
  },

  toggleFeature: async (featureId, enabled) => {
    const { features } = get()

    // Optimistic update
    set({
      features: features.map((f) =>
        f.id === featureId ? { ...f, enabled } : f
      ),
    })

    try {
      await featuresApi.toggleFeature(featureId, enabled)
    } catch (error) {
      // Revert on error
      set({
        features: features.map((f) =>
          f.id === featureId ? { ...f, enabled: !enabled } : f
        ),
        error: error instanceof Error ? error.message : 'Failed to toggle feature',
      })
    }
  },

  isFeatureEnabled: (featureKey) => {
    const { features } = get()
    const feature = features.find((f) => f.key === featureKey)
    return feature?.enabled ?? false
  },
}))
