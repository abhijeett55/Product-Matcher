import { useState } from 'react';
import type { Product } from '../types/product';
import { productApi } from '../services/api';

export const useProducts = () => {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const searchSimilarProducts = async (imageUrl?: string, imageFile?: File) => {
    setLoading(true);
    setError(null);
    
    try {
      let searchData: { image_url?: string; image_data?: string } = {};
      
      if (imageUrl) {
        searchData.image_url = imageUrl;
      } else if (imageFile) {
        const base64Data = await uploadImageToBase64(imageFile);
        searchData.image_data = base64Data;
      } else {
        throw new Error('Either image URL or file must be provided');
      }

      const response = await productApi.searchSimilarProducts(searchData);
      setProducts(response.similar_products);
      return response;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Search failed';
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const uploadImageToBase64 = (file: File): Promise<string> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => {
        const base64 = reader.result as string;
        const base64Data = base64.split(',')[1];
        resolve(base64Data);
      };
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });
  };

  const clearResults = () => {
    setProducts([]);
    setError(null);
  };

  return {
    products,
    loading,
    error,
    searchSimilarProducts,
    clearResults,
  };
};