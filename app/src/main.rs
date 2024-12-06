use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod errors;
mod handlers;
mod models;
use crate::handlers::{travel_entry, upload, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing::info!("Connecting to database: {}", database_url);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

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
            .service(
                web::scope("/travel-entries")
                    .route("", web::post().to(travel_entry::create_travel_entry))
                    .route(
                        "/user/{user_id}",
                        web::get().to(travel_entry::get_travel_entry),
                    )
                    .route("/{id}", web::get().to(travel_entry::get_travel_entry))
                    .route("/{id}", web::put().to(travel_entry::update_travel_entry))
                    .route("/{id}", web::delete().to(travel_entry::delete_travel_entry)),
            )
            .service(web::scope("/uploads").route(
                "/presigned-url",
                web::post().to(upload::generate_presigned_url),
            ))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
