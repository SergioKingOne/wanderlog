use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TravelEntry {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "visitDate")]
    pub visit_date: DateTime<Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTravelEntry {
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "visitDate")]
    pub visit_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTravelEntry {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "visitDate")]
    pub visit_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TravelEntryImage {
    pub id: i32,
    pub travel_entry_id: i32,
    pub image_key: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddTravelEntryImage {
    pub image_key: String,
}
