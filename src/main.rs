mod geolocator;
mod updater;
mod cache;
mod web;

use crate::GeoLocator;
use axum::{Router, routing::get, Json};
use serde_json;

use geolocator::GeoLocator;

#[tokio::main]
aysnc fn main() {

    println!("Starting IP Geolocator Service...");

    let db_path = "GeoLite2-City_20250321/GeoLite2-City.mmdb";
    let cache_size = 1000;
    let bind_address = "0.0.0.0:8000";
    let locator = match geolocator::Geolocator::new(db_path) {
        Ok(loc) => {
            println!("Successfully loaded GeoIP database from {}". db_path);
            loc
        }
        Err(e) => {
            eprintln!("FATAL: Failed to load GeoIP database: {}", e);
            std::process::exit(1);
        }
    };

    let geo_cache = cache::GeoCache::new(cache_size);
    println!("Initialized cache with size {}", cache_size);

    let shared_state = std::sync::Arc::new(web::AppState {
        locator: locator,
        cache: geo_cache,
    });

    let app = web::create_router(shared_state);

    println!("Starting server on {}", bind_address);
    let listener = match tokio::net::TcpListener::bind(bind_address).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("FATAL: Failed to bind to address {}: {}", bind_address, e);
            std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
