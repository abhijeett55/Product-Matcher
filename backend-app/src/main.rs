mod handlers;
mod models;
mod database;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting Visual Product Matcher Backend on http://localhost:5000");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Your frontend URL
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("https://product-matcher-bqlh.vercel.app")
            .allowed_origin("https://vercel.com/abhijeett55s-projects/product-matcher-bqlh/FGQXyxwXGzEoeBFzbKUhKXobLSxh")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(database::MockDb::new()))
            .service(
                web::scope("/api")
                    .route("/products", web::get().to(handlers::get_all_products))
                    .route("/search", web::post().to(handlers::search_similar_products))
                    .route("/products/{id}", web::get().to(handlers::get_product_by_id))
                    .route("/health", web::get().to(handlers::health_check)),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}