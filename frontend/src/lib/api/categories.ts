import { api } from './client'
import type { Category } from '@/types'

interface CreateCategoryParams {
  name: string
  color: string
  icon?: string
  categoryType: 'income' | 'expense'
}

interface UpdateCategoryParams {
  name?: string
  color?: string
  icon?: string
}

export const categoriesApi = {
  list: () => api.get<Category[]>('/v1/categories/'),

  listByType: (categoryType: 'income' | 'expense') =>
    api.get<Category[]>(`/v1/categories/type/${categoryType}`),

  create: (params: CreateCategoryParams) =>
    api.post<Category>('/v1/categories/', {
      name: params.name,
      color: params.color,
      icon: params.icon,
      category_type: params.categoryType,
    }),

  update: (id: number, params: UpdateCategoryParams) =>
    api.put<Category>(`/v1/categories/${id}`, params),

  delete: (id: number) =>
    api.delete<{ status: string }>(`/v1/categories/${id}`),
}
