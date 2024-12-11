use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use aws_sdk_s3::presigning::PresigningConfig;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PresignedUrlRequest {
    file_type: String,
}

#[derive(Serialize)]
pub struct PresignedUrlResponse {
    upload_url: String,
    key: String,
}

#[derive(Deserialize)]
pub struct GetPresignedUrlRequest {
    key: String,
}

#[derive(Serialize)]
pub struct GetPresignedUrlResponse {
    download_url: String,
}

pub async fn generate_presigned_url(
    req: web::Json<PresignedUrlRequest>,
) -> Result<HttpResponse, AppError> {
    // Initialize AWS SDK
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::v2024_03_28()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    // Generate unique filename
    let file_extension = req.file_type.split('/').last().unwrap_or("jpg");
    let key = format!("uploads/{}.{}", Uuid::new_v4(), file_extension);

    // Create presigned URL that expires in 15 minutes
    let presigned_request = s3_client
        .put_object()
        .bucket(std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set"))
        .key(&key)
        .content_type(&req.file_type)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(900))?)
        .await
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(PresignedUrlResponse {
        upload_url: presigned_request.uri().to_string(),
        key,
    }))
}

pub async fn generate_download_presigned_url(
    req: web::Json<GetPresignedUrlRequest>,
) -> Result<HttpResponse, AppError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::v2024_03_28()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    // Create presigned URL that expires in 15 minutes
    let presigned_request = s3_client
        .get_object()
        .bucket(std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set"))
        .key(&req.key)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(900))?)
        .await
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(GetPresignedUrlResponse {
        download_url: presigned_request.uri().to_string(),
    }))
}
