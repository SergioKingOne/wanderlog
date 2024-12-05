use crate::errors::AppError;
use crate::models::travel_entry::{CreateTravelEntry, TravelEntry, UpdateTravelEntry};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn create_travel_entry(
    pool: web::Data<PgPool>,
    entry: web::Json<CreateTravelEntry>,
) -> Result<HttpResponse, AppError> {
    // Validate input
    if entry.title.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Title cannot be empty".to_string(),
        ));
    }

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
    .await?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn update_travel_entry(
    pool: web::Data<PgPool>,
    entry_id: web::Path<i32>,
    update: web::Json<UpdateTravelEntry>,
) -> Result<HttpResponse, AppError> {
    let entry_id_value = entry_id.into_inner();

    let entry = sqlx::query_as!(
        TravelEntry,
        r#"
        SELECT * FROM travel_entries WHERE id = $1
        "#,
        entry_id_value
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Travel entry not found".to_string()))?;

    let updated = sqlx::query_as!(
        TravelEntry,
        r#"
        UPDATE travel_entries
        SET 
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            location = COALESCE($3, location),
            latitude = COALESCE($4, latitude),
            longitude = COALESCE($5, longitude),
            visit_date = COALESCE($6, visit_date),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $7
        RETURNING id, user_id, title, description, location, latitude, longitude, visit_date, created_at, updated_at
        "#,
        update.title,
        update.description,
        update.location,
        update.latitude,
        update.longitude,
        update.visit_date,
        entry_id_value
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete_travel_entry(
    pool: web::Data<PgPool>,
    entry_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM travel_entries WHERE id = $1
        RETURNING id
        "#,
        entry_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(AppError::NotFound("Travel entry not found".to_string())),
    }
}

pub async fn get_travel_entry(
    pool: web::Data<PgPool>,
    entry_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let entry = sqlx::query_as!(
        TravelEntry,
        r#"
        SELECT id, user_id, title, description, location, latitude, longitude, visit_date, created_at, updated_at
        FROM travel_entries
        WHERE id = $1
        "#,
        entry_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Travel entry not found".to_string()))?;

    Ok(HttpResponse::Ok().json(entry))
}
