use actix_web::{web, HttpResponse};
use crate::database::MockDb;
use crate::models::product::{ApiResponse, SearchRequest, SearchResponse, EmptyData};

pub async fn search_similar_products(
    db: web::Data<MockDb>,
    search_req: web::Json<SearchRequest>,
) -> HttpResponse {
    // Validate input
    if search_req.image_url.is_none() && search_req.image_data.is_none() {
        return HttpResponse::BadRequest().json(ApiResponse::<EmptyData>::error(
            "Either image_url or image_data must be provided".to_string()
        ));
    }

    let query_image = if let Some(url) = &search_req.image_url {
        url.clone()
    } else if let Some(data) = &search_req.image_data {
        // For base64 data, we'd process it differently in a real implementation
        format!("data:image/jpeg;base64,{}", data)
    } else {
        return HttpResponse::BadRequest().json(ApiResponse::<EmptyData>::error(
            "No image data provided".to_string()
        ));
    };

    // Search for similar products
    let similar_products = db.search_similar_products(&query_image);
    let total_matches = similar_products.len();

    let response = SearchResponse {
        query_image,
        similar_products,
        total_matches,
    };

    HttpResponse::Ok().json(ApiResponse::success(response))
}