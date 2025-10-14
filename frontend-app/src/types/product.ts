export interface Product {
  id: string;
  name: string;
  category: string;
  price: number;
  image_url: string;
  description: string;
  similarity_score?: number;
}

export interface SearchResponse {
  query_image: string;
  similar_products: Product[];
  total_matches: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T;
  message?: string;
}

export interface SearchRequest {
  image_url?: string;
  image_data?: string;
}