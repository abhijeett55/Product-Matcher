import React from 'react';
import type { Product } from '../../types/product';
import './ProductCard.css';

interface ProductCardProps {
  product: Product;
}

const ProductCard: React.FC<ProductCardProps> = ({ product }) => {
  const similarityPercentage = product.similarity_score 
    ? Math.round(product.similarity_score * 100)
    : 0;

  const getSimilarityColor = (score: number) => {
    if (score >= 0.8) return '#51cf66';
    if (score >= 0.6) return '#ffd43b';
    if (score >= 0.4) return '#ffa94d';
    return '#ff6b6b';
  };

  return (
    <div className="product-card">
      <div className="product-image">
        <img src={product.image_url} alt={product.name} />
        <div 
          className="similarity-badge"
          style={{ backgroundColor: getSimilarityColor(product.similarity_score || 0) }}
        >
          {similarityPercentage}% Match
        </div>
      </div>
      
      <div className="product-info">
        <h3 className="product-name">{product.name}</h3>
        <p className="product-category">{product.category}</p>
        <p className="product-description">{product.description}</p>
        <div className="product-price">${product.price.toFixed(2)}</div>
      </div>
    </div>
  );
};

export default ProductCard;