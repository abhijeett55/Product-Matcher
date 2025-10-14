import React, { useState, useCallback } from 'react';
import ImageUpload from './components/ImageUpload/ImageUpload';
import ProductGrid from './components/ProductGrid/ProductGrid';
import Loading from './components/Loading/Loading';
import { useProducts } from './hooks/useProducts';
import type { Product } from './types/product';
import './App.css';

const App: React.FC = () => {
  const { products, loading, error, searchSimilarProducts, clearResults } = useProducts();
  const [queryImage, setQueryImage] = useState<string>('');
  const [filteredProducts, setFilteredProducts] = useState<Product[]>([]);

  const handleImageUpload = async (imageUrl?: string, imageFile?: File) => {
    try {
      const response = await searchSimilarProducts(imageUrl, imageFile);
      setQueryImage(response.query_image);
      setFilteredProducts(response.similar_products);
    } catch (err) {
      // Error handled by hook
    }
  };

  const handleFilterChange = useCallback((minScore: number) => {
    if (minScore === 0) {
      setFilteredProducts(products);
    } else {
      const filtered = products.filter(
        product => (product.similarity_score || 0) >= minScore
      );
      setFilteredProducts(filtered);
    }
  }, [products]);

  const handleClearResults = () => {
    clearResults();
    setQueryImage('');
    setFilteredProducts([]);
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>🛍️ Visual Product Matcher</h1>
        <p>Find similar products using AI-powered visual search</p>
      </header>

      <main className="app-main">
        <ImageUpload onImageUpload={handleImageUpload} loading={loading} />
        
        {error && (
          <div className="error-message">
            ❌ {error}
            <button onClick={handleClearResults} className="clear-results-btn">
              Try Again
            </button>
          </div>
        )}

        {loading && <Loading />}

        {!loading && filteredProducts.length > 0 && (
          <ProductGrid
            products={filteredProducts}
            queryImage={queryImage}
            onFilterChange={handleFilterChange}
          />
        )}

        {!loading && products.length > 0 && (
          <div className="results-actions">
            <button onClick={handleClearResults} className="clear-results-btn">
              Clear Results
            </button>
          </div>
        )}
      </main>

      <footer className="app-footer">
        <p>Visual Product Matcher &copy; 2024</p>
      </footer>
    </div>
  );
};

export default App;