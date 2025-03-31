use crate::{cache::GeoCache, geolocator::{GeoData, GeoLocator}}:
use axum::{
    extract::{Path, State}, 
    http::StatusCode,
    response::{IntoResponse, Response}
    routing::get,
    Json, Router,
};
use std::net::IpAddr;
use std::sync::Arc;

pub struct AppState {
    pub locator: Geolocator,
    pub cache: GeoCache,
}

pub fn create_router(shared_router(shared_state: Arc<AppState>) -> Router {
    Router::new().route("/loookup/:ip", get(handle_lookup))
                .route("/health", get(|| async { "OK" }))
                .with_state(shared_state)
})

async fn handle_lookup(
    path(ip_str): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<GeoData>, AppError> {
    let ip: IpAddr = ip_str.parse().map_err(|_| {AppError::BadRequest("Invalid Ip address format".to_string())
})?;

if let Some(cached_result_str) = state.cache.get(ip) {
    let data: GeoData = serde_json::from_str(&cached_result_str).map_err(|e| AppError::Internal(format!("Cache deserialization error: {}", e)))?;
    println!("Cache hit for {}", ip);

    return Ok(Json(data));
}

println!("Cache miss for {}", ip);

let result = state.locator.lookup(&ip_str).map_err(|e| {
    eprintln!("Lookup error for IP {}: {}", ip_str, e);

    AppError::Internal("Failed to lookup IP address".to_string())
})?;

match serde_json::to_string(&result) {
    Ok(json_string) => {
        state.cache.insert(ip, json_string);
    }
    Err(e) => {
        eprintln!("Failed to serialize result for caching IP{}: {}", ip, e);
    }
}

 Ok(Json(result))
}

enum AppError {
    Internal(String),
    BadRequest(String),
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
      let (status, error_message) = match self {
        AppError: Internal(msg) => {
            eprintln!("Internal Server Error: {}", msg);
            (statusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
        }
        AppError: BadRequest(msg) => {
            eprintln!("Bad Request: {}", msg);
            (StatusCode::BAD_REQUEST, msg)
        }
        AppError: NotFound(msg) => {
            eprintln!("Not Found: {}", msg);
            (StatusCode::NOT_FOUND, msg)
        };

        (status, Json(serde_json::json!({ "error": error_message}))).into_response()
      }  
    }
}