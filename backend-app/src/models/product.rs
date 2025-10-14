use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub image_url: String,
    pub description: String,
    pub similarity_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub image_url: Option<String>,
    pub image_data: Option<String>, // base64 encoded image
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query_image: String,
    pub similar_products: Vec<Product>,
    pub total_matches: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

// Add default implementations for common types
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyData;

impl Default for EmptyData {
    fn default() -> Self {
        EmptyData
    }
}

impl Product {
    pub fn new(
        name: String,
        category: String,
        price: f64,
        image_url: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            category,
            price,
            image_url,
            description,
            similarity_score: None,
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }
}

// Separate error method that doesn't require T: Default
impl ApiResponse<EmptyData> {
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: EmptyData,
            message: Some(message),
        }
    }
}