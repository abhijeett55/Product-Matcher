use super::super::models::product::Product;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct MockDb {
    products: RwLock<HashMap<String, Product>>,
}

impl MockDb {
    pub fn new() -> Self {
        let products = RwLock::new(HashMap::new());
        let db = Self { products };
        db.initialize_sample_data();
        db
    }

    fn initialize_sample_data(&self) {
        let sample_products = vec![
            // Footwear (15 products)
            Product::new("Classic White Sneakers".to_string(), "Footwear".to_string(), 79.99, "https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=400".to_string(), "Comfortable white sneakers for everyday wear".to_string()),
            Product::new("Sports Running Shoes".to_string(), "Footwear".to_string(), 129.99, "https://images.unsplash.com/photo-1606107557195-0e29a4b5b4aa?w=400".to_string(), "High-performance running shoes".to_string()),
            Product::new("Black Leather Boots".to_string(), "Footwear".to_string(), 149.99, "https://images.unsplash.com/photo-1549317661-bd32c8ce0db2?w=400".to_string(), "Stylish black leather boots".to_string()),
            Product::new("Casual Loafers".to_string(), "Footwear".to_string(), 89.99, "https://images.unsplash.com/photo-1560769684-55015cee73a8?w=400".to_string(), "Comfortable casual loafers".to_string()),
            Product::new("Basketball Shoes".to_string(), "Footwear".to_string(), 119.99, "https://images.unsplash.com/photo-1587563871167-1ee9c731aefb?w=400".to_string(), "High-top basketball shoes".to_string()),
            Product::new("Hiking Boots".to_string(), "Footwear".to_string(), 159.99, "https://images.unsplash.com/photo-1605348530036-7fec2d0e8d8c?w=400".to_string(), "Durable hiking boots".to_string()),
            Product::new("Sandals".to_string(), "Footwear".to_string(), 39.99, "https://images.unsplash.com/photo-1562279588-7b6c06be9ce8?w=400".to_string(), "Summer sandals".to_string()),
            Product::new("Formal Oxfords".to_string(), "Footwear".to_string(), 179.99, "https://images.unsplash.com/photo-1525966222134-fcfa99b8ae77?w=400".to_string(), "Elegant formal oxford shoes".to_string()),
            Product::new("Slip-on Sneakers".to_string(), "Footwear".to_string(), 69.99, "https://images.unsplash.com/photo-1575537302964-96d34c3e4eb5?w=400".to_string(), "Easy slip-on sneakers".to_string()),
            Product::new("Winter Boots".to_string(), "Footwear".to_string(), 129.99, "https://images.unsplash.com/photo-1548123370-91f4ea8a43b0?w=400".to_string(), "Warm winter boots".to_string()),
            Product::new("Platform Sneakers".to_string(), "Footwear".to_string(), 99.99, "https://images.unsplash.com/photo-1605034313761-73ea4a0cfbf3?w=400".to_string(), "Trendy platform sneakers".to_string()),
            Product::new("Dress Shoes".to_string(), "Footwear".to_string(), 139.99, "https://images.unsplash.com/photo-1463100099107-aa0980c362e6?w=400".to_string(), "Classic dress shoes".to_string()),
            Product::new("Canvas Shoes".to_string(), "Footwear".to_string(), 49.99, "https://images.unsplash.com/photo-1525966222134-fcfa99b8ae77?w=400".to_string(), "Lightweight canvas shoes".to_string()),
            Product::new("Athletic Sandals".to_string(), "Footwear".to_string(), 59.99, "https://images.unsplash.com/photo-1575537302964-96d34c3e4eb5?w=400".to_string(), "Sporty athletic sandals".to_string()),
            Product::new("Espadrilles".to_string(), "Footwear".to_string(), 45.99, "https://images.unsplash.com/photo-1562279588-7b6c06be9ce8?w=400".to_string(), "Summer espadrilles".to_string()),

            // Outerwear (10 products)
            Product::new("Black Leather Jacket".to_string(), "Outerwear".to_string(), 199.99, "https://images.unsplash.com/photo-1551028719-00167b16eac5?w=400".to_string(), "Premium black leather jacket".to_string()),
            Product::new("Winter Wool Coat".to_string(), "Outerwear".to_string(), 299.99, "https://images.unsplash.com/photo-1551488831-00ddcb929696?w=400".to_string(), "Warm winter wool coat".to_string()),
            Product::new("Denim Jacket".to_string(), "Outerwear".to_string(), 89.99, "https://images.unsplash.com/photo-1551698618-1dfe5d97d256?w=400".to_string(), "Classic denim jacket".to_string()),
            Product::new("Rain Jacket".to_string(), "Outerwear".to_string(), 79.99, "https://images.unsplash.com/photo-1551028719-00167b16eac5?w=400".to_string(), "Waterproof rain jacket".to_string()),
            Product::new("Bomber Jacket".to_string(), "Outerwear".to_string(), 129.99, "https://images.unsplash.com/photo-1591047139829-d91aecb6caea?w=400".to_string(), "Stylish bomber jacket".to_string()),
            Product::new("Trench Coat".to_string(), "Outerwear".to_string(), 249.99, "https://images.unsplash.com/photo-1539533018447-63fcce2678e7?w=400".to_string(), "Elegant trench coat".to_string()),
            Product::new("Puffer Jacket".to_string(), "Outerwear".to_string(), 179.99, "https://images.unsplash.com/photo-1551488831-00ddcb929696?w=400".to_string(), "Warm puffer jacket".to_string()),
            Product::new("Blazer".to_string(), "Outerwear".to_string(), 159.99, "https://images.unsplash.com/photo-1593030103066-0093718efeb9?w=400".to_string(), "Formal blazer".to_string()),
            Product::new("Windbreaker".to_string(), "Outerwear".to_string(), 69.99, "https://images.unsplash.com/photo-1551028719-00167b16eac5?w=400".to_string(), "Lightweight windbreaker".to_string()),
            Product::new("Parka".to_string(), "Outerwear".to_string(), 219.99, "https://images.unsplash.com/photo-1551488831-00ddcb929696?w=400".to_string(), "Winter parka with fur hood".to_string()),

            // Tops (10 products)
            Product::new("Casual T-Shirt".to_string(), "Tops".to_string(), 24.99, "https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=400".to_string(), "Comfortable cotton t-shirt".to_string()),
            Product::new("Formal Dress Shirt".to_string(), "Tops".to_string(), 49.99, "https://images.unsplash.com/photo-1596755094514-f87e34085b2c?w=400".to_string(), "Elegant formal dress shirt".to_string()),
            Product::new("Polo Shirt".to_string(), "Tops".to_string(), 34.99, "https://images.unsplash.com/photo-1586790170083-2f9ceadc732d?w=400".to_string(), "Classic polo shirt".to_string()),
            Product::new("Sweater".to_string(), "Tops".to_string(), 59.99, "https://images.unsplash.com/photo-1434389677669-e08b4cac3105?w=400".to_string(), "Warm knit sweater".to_string()),
            Product::new("Hoodie".to_string(), "Tops".to_string(), 44.99, "https://images.unsplash.com/photo-1556821840-3a63f95609a7?w=400".to_string(), "Comfortable hoodie".to_string()),
            Product::new("Blouse".to_string(), "Tops".to_string(), 39.99, "https://images.unsplash.com/photo-1594633312681-425c7b97ccd1?w=400".to_string(), "Elegant blouse".to_string()),
            Product::new("Tank Top".to_string(), "Tops".to_string(), 19.99, "https://images.unsplash.com/photo-1503342217505-b0a15ec3261c?w=400".to_string(), "Summer tank top".to_string()),
            Product::new("Long Sleeve Shirt".to_string(), "Tops".to_string(), 29.99, "https://images.unsplash.com/photo-1503342217505-b0a15ec3261c?w=400".to_string(), "Casual long sleeve shirt".to_string()),
            Product::new("Crop Top".to_string(), "Tops".to_string(), 22.99, "https://images.unsplash.com/photo-1503342217505-b0a15ec3261c?w=400".to_string(), "Trendy crop top".to_string()),
            Product::new("Button Down Shirt".to_string(), "Tops".to_string(), 54.99, "https://images.unsplash.com/photo-1596755094514-f87e34085b2c?w=400".to_string(), "Classic button down shirt".to_string()),

            // Bottoms (10 products)
            Product::new("Blue Denim Jeans".to_string(), "Bottoms".to_string(), 59.99, "https://images.unsplash.com/photo-1542272604-787c3835535d?w=400".to_string(), "Classic blue denim jeans".to_string()),
            Product::new("Yoga Pants".to_string(), "Bottoms".to_string(), 39.99, "https://images.unsplash.com/photo-1506629905607-e48b0e67d879?w=400".to_string(), "Comfortable yoga pants".to_string()),
            Product::new("Chino Pants".to_string(), "Bottoms".to_string(), 49.99, "https://images.unsplash.com/photo-1473966968600-fa801b869a1a?w=400".to_string(), "Casual chino pants".to_string()),
            Product::new("Shorts".to_string(), "Bottoms".to_string(), 34.99, "https://images.unsplash.com/photo-1503342217505-b0a15ec3261c?w=400".to_string(), "Summer shorts".to_string()),
            Product::new("Dress Pants".to_string(), "Bottoms".to_string(), 79.99, "https://images.unsplash.com/photo-1473966968600-fa801b869a1a?w=400".to_string(), "Formal dress pants".to_string()),
            Product::new("Cargo Pants".to_string(), "Bottoms".to_string(), 54.99, "https://images.unsplash.com/photo-1542272604-787c3835535d?w=400".to_string(), "Utility cargo pants".to_string()),
            Product::new("Leggings".to_string(), "Bottoms".to_string(), 29.99, "https://images.unsplash.com/photo-1506629905607-e48b0e67d879?w=400".to_string(), "Comfortable leggings".to_string()),
            Product::new("Skirt".to_string(), "Bottoms".to_string(), 44.99, "https://images.unsplash.com/photo-1595777457583-95e059d581b8?w=400".to_string(), "Elegant skirt".to_string()),
            Product::new("Joggers".to_string(), "Bottoms".to_string(), 49.99, "https://images.unsplash.com/photo-1542272604-787c3835535d?w=400".to_string(), "Comfortable jogger pants".to_string()),
            Product::new("Palazzo Pants".to_string(), "Bottoms".to_string(), 64.99, "https://images.unsplash.com/photo-1473966968600-fa801b869a1a?w=400".to_string(), "Flowy palazzo pants".to_string()),

            // Dresses (5 products)
            Product::new("Red Summer Dress".to_string(), "Dresses".to_string(), 89.99, "https://images.unsplash.com/photo-1595777457583-95e059d581b8?w=400".to_string(), "Elegant red summer dress".to_string()),
            Product::new("Black Cocktail Dress".to_string(), "Dresses".to_string(), 129.99, "https://images.unsplash.com/photo-1515372039744-b8f02a3ae446?w=400".to_string(), "Little black dress".to_string()),
            Product::new("Maxi Dress".to_string(), "Dresses".to_string(), 79.99, "https://images.unsplash.com/photo-1469334031218-e382a71b716b?w=400".to_string(), "Long maxi dress".to_string()),
            Product::new("Floral Dress".to_string(), "Dresses".to_string(), 69.99, "https://images.unsplash.com/photo-1595777457583-95e059d581b8?w=400".to_string(), "Beautiful floral dress".to_string()),
            Product::new("Evening Gown".to_string(), "Dresses".to_string(), 199.99, "https://images.unsplash.com/photo-1515372039744-b8f02a3ae446?w=400".to_string(), "Elegant evening gown".to_string()),

            // Accessories (5 products)
            Product::new("Designer Handbag".to_string(), "Accessories".to_string(), 199.99, "https://images.unsplash.com/photo-1584917865442-de89df76afd3?w=400".to_string(), "Luxury designer handbag".to_string()),
            Product::new("Sunglasses".to_string(), "Accessories".to_string(), 89.99, "https://images.unsplash.com/photo-1572635196237-14b3f281503f?w=400".to_string(), "Stylish sunglasses".to_string()),
            Product::new("Watch".to_string(), "Accessories".to_string(), 149.99, "https://images.unsplash.com/photo-1523170335258-f5ed11844a49?w=400".to_string(), "Elegant wristwatch".to_string()),
            Product::new("Scarf".to_string(), "Accessories".to_string(), 34.99, "https://images.unsplash.com/photo-1548036328-c9fa89d128fa?w=400".to_string(), "Warm winter scarf".to_string()),
            Product::new("Baseball Cap".to_string(), "Accessories".to_string(), 24.99, "https://images.unsplash.com/photo-1588850561407-ed78c282e89b?w=400".to_string(), "Casual baseball cap".to_string()),
        ];

        let mut products = self.products.write().unwrap();
        for product in sample_products {
            products.insert(product.id.clone(), product);
        }
        
        println!("Initialized database with {} products", products.len());
    }

    pub fn get_all_products(&self) -> Vec<Product> {
        let products = self.products.read().unwrap();
        products.values().cloned().collect()
    }

    pub fn get_product_by_id(&self, id: &str) -> Option<Product> {
        let products = self.products.read().unwrap();
        products.get(id).cloned()
    }

    pub fn search_similar_products(&self, query_image: &str) -> Vec<Product> {
        let mut products = self.get_all_products();
        
        // Simulate similarity scoring based on image characteristics
        for product in &mut products {
            let similarity = self.calculate_similarity(query_image, &product.image_url);
            product.similarity_score = Some(similarity);
        }

        // Sort by similarity score (descending)
        products.sort_by(|a, b| {
            b.similarity_score
                .partial_cmp(&a.similarity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top 10 most similar products
        products.into_iter().take(10).collect()
    }

    fn calculate_similarity(&self, query_image: &str, target_image: &str) -> f64 {
        // Simple mock similarity calculation
        let query_hash = self.image_to_hash(query_image);
        let target_hash = self.image_to_hash(target_image);
        
        let distance = self.hamming_distance(&query_hash, &target_hash);
        let max_distance = 64.0;
        (max_distance - distance as f64) / max_distance
    }

    fn image_to_hash(&self, image_url: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        image_url.hash(&mut hasher);
        hasher.finish()
    }

    fn hamming_distance(&self, a: &u64, b: &u64) -> u32 {
        (a ^ b).count_ones()
    }
}