export interface LiveProductPreview {
  name: string
  price: number
}

export interface LivePreview {
  id: string
  sellerName: string
  sellerHandle: string
  title: string
  category: LiveCategoryId
  location: string
  viewerCount: number
  videoUrl: string
  tiktokUsername: string
  featured: boolean
  product: LiveProductPreview
}

export type LiveCategoryId = 'mode' | 'beauty' | 'food' | 'vehicles' | 'home'

export interface LiveCategory {
  id: LiveCategoryId
  label: string
  description: string
}
