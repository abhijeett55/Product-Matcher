use actix_web::{web, HttpResponse};
use crate::database::MockDb;
use crate::models::product::{ApiResponse, EmptyData};

pub async fn get_all_products(db: web::Data<MockDb>) -> HttpResponse {
    let products = db.get_all_products();
    HttpResponse::Ok().json(ApiResponse::success(products))
}

pub async fn get_product_by_id(
    db: web::Data<MockDb>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    
    match db.get_product_by_id(&id) {
        Some(product) => HttpResponse::Ok().json(ApiResponse::success(product)),
        None => HttpResponse::NotFound().json(ApiResponse::<EmptyData>::error(
            "Product not found".to_string()
        )),
    }
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Visual Product Matcher API"
    }))
}