use axum::{
    extract::{Json, Path, State},
    response::Redirect,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{ShortenRequest, ShortenResponse},
    state::AppState,
};

pub async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    let code = Uuid::new_v4().to_string()[..6].to_string();

    sqlx::query!(
        "INSERT INTO urls (code, original_url) VALUES (?, ?)",
        code,
        payload.url
    )
    .execute(&state.db)
    .await?;

    Ok(Json(ShortenResponse {
        short_url: format!("http://localhost:3000/{}", code),
    }))
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, AppError> {
    let record = sqlx::query!("SELECT original_url FROM urls WHERE code = ?", code)
        .fetch_optional(&state.db)
        .await?;

    let url = record.ok_or(AppError::NotFound)?.original_url;

    Ok(Redirect::permanent(&url))
}
