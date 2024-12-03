use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::user::{CreateUser, User};

pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    if user.name.is_empty() || user.email.is_empty() {
        return Err(AppError::ValidationError(
            "Name and email are required".into(),
        ));
    }

    let user = sqlx::query!(
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email, created_at, updated_at
        "#,
        user.name,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await?;

    let user = User {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(HttpResponse::Created().json(user))
}

pub async fn get_users(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let users = sqlx::query!(
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await?
    .into_iter()
    .map(|row| User {
        id: row.id,
        name: row.name,
        email: row.email,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
    .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let user = sqlx::query!(
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match user {
        Some(row) => {
            let user = User {
                id: row.id,
                name: row.name,
                email: row.email,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            Ok(HttpResponse::Ok().json(user))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "message": "User not found"
        }))),
    }
}
