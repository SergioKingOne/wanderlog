use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod errors;
mod handlers;
mod models;
use crate::handlers::user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database: {}", database_url);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Test the connection
    let result = sqlx::query!("SELECT current_database() as db")
        .fetch_one(&pool)
        .await
        .expect("Failed to query database");

    println!("Connected to database: {}", result.db.unwrap_or_default());

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api").service(
                    web::scope("/users")
                        .route("", web::post().to(user::create_user))
                        .route("", web::get().to(user::get_users))
                        .route("/{id}", web::get().to(user::get_user_by_id)),
                ),
            )
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
