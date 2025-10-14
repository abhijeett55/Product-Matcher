import axios from 'axios';
import type { Product, SearchResponse, ApiResponse, SearchRequest } from '../types/product';

const API_BASE_URL = 'http://localhost:5000/api'|| 'https://product-matcher-production-21d7.up.railway.app/api' ;


const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 30000,
});

// api.interceptors.response.use(
//   (response) => {
//     console.log('✅ Response received:', response.status);
//     return response;
//   },
//   (error) => {
//     console.error('❌ API Error:', {
//       status: error.response?.status,
//       message: error.message,
//       url: error.config?.url
//     });
    
//     if (error.code === 'ECONNREFUSED' || error.message.includes('Network Error')) {
//       throw new Error(`Cannot connect to backend server at ${API_BASE_URL}. Please make sure the backend is running.`);
//     }
//     if (error.response) {
//       // Server responded with error status
//       const status = error.response.status;
//       if (status === 404) {
//         throw new Error('Backend endpoint not found. The server might be down.');
//       } else if (status >= 500) {
//         throw new Error('Backend server error. Please try again later.');
//       }
//     }
    
//     throw error;
//   }
// );





export const productApi = {
  // Get all products
  getAllProducts: async (): Promise<Product[]> => {
    const response = await api.get<ApiResponse<Product[]>>('/products');
    return response.data.data;
  },

  // Get product by ID
  getProductById: async (id: string): Promise<Product> => {
    const response = await api.get<ApiResponse<Product>>(`/products/${id}`);
    return response.data.data;
  },

  // Search similar products
  searchSimilarProducts: async (searchData: SearchRequest): Promise<SearchResponse> => {
    const response = await api.post<ApiResponse<SearchResponse>>('/search', searchData);
    return response.data.data;
  },

  // Health check
  healthCheck: async (): Promise<{ status: string; service: string }> => {
    const response = await api.get('/health');
    return response.data;
  },
};

export const uploadImageToBase64 = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const base64 = reader.result as string;
      // Remove data:image/...;base64, prefix
      const base64Data = base64.split(',')[1];
      resolve(base64Data);
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
};