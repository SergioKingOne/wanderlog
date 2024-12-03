use crate::models::travel_entry::{CreateTravelEntry, TravelEntry};
use actix_web::{web, Error, HttpResponse};
use sqlx::PgPool;

pub async fn create_travel_entry(
    pool: web::Data<PgPool>,
    entry: web::Json<CreateTravelEntry>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(
        TravelEntry,
        r#"
        INSERT INTO travel_entries (user_id, title, description, location, latitude, longitude, visit_date, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, user_id, title, description, location, latitude, longitude, visit_date, created_at, updated_at
        "#,
        entry.user_id,
        entry.title,
        entry.description,
        entry.location,
        entry.latitude,
        entry.longitude,
        entry.visit_date,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(entry) => Ok(HttpResponse::Created().json(entry)),
        Err(e) => {
            eprintln!("Failed to create travel entry: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn get_user_travel_entries(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(
        TravelEntry,
        r#"
        SELECT id, user_id, title, description, location, latitude, longitude, visit_date, created_at, updated_at
        FROM travel_entries
        WHERE user_id = $1
        ORDER BY visit_date DESC
        "#,
        user_id.into_inner()
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(entries) => Ok(HttpResponse::Ok().json(entries)),
        Err(e) => {
            eprintln!("Failed to fetch travel entries: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
