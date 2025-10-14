import React from 'react';
import type { Product } from '../../types/product';
import ProductCard from '../ProductCard/ProductCard';
import './ProductGrid.css';

interface ProductGridProps {
  products: Product[];
  queryImage?: string;
  onFilterChange: (minScore: number) => void;
}

const ProductGrid: React.FC<ProductGridProps> = ({ 
  products, 
  queryImage, 
  onFilterChange 
}) => {
  const handleFilterChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const minScore = parseFloat(event.target.value);
    onFilterChange(minScore);
  };

  if (products.length === 0) {
    return (
      <div className="product-grid-empty">
        <p>No products found. Upload an image to find similar products!</p>
      </div>
    );
  }

  return (
    <div className="product-grid">
      <div className="product-grid-header">
        {queryImage && (
          <div className="query-image-section">
            <h3>Search Image</h3>
            <img src={queryImage} alt="Search query" className="query-image" />
          </div>
        )}
        
        <div className="results-info">
          <h3>Similar Products ({products.length})</h3>
          <div className="filter-control">
            <label htmlFor="similarity-filter">Min Similarity:</label>
            <select 
              id="similarity-filter" 
              onChange={handleFilterChange}
              defaultValue="0"
            >
              <option value="0">All</option>
              <option value="0.3">30%+</option>
              <option value="0.5">50%+</option>
              <option value="0.7">70%+</option>
              <option value="0.9">90%+</option>
            </select>
          </div>
        </div>
      </div>

      <div className="products-container">
        {products.map((product) => (
          <ProductCard key={product.id} product={product} />
        ))}
      </div>
    </div>
  );
};

export default ProductGrid;