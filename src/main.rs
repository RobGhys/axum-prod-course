#![allow(unused)]

mod ctx;
mod error;
mod log;
mod model;
mod web;
mod config;
mod crypt;
mod utils;
// #[cfg(test)] --> commented during early development
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config; // We can now use --> use crate::config;

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_response_map;
use crate::web::{routes_login, routes_static};

use std::net::SocketAddr;
use axum::{middleware, Router};
use axum::response::Html;
use axum::routing::get;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- For DEV only
    _dev_utils::init_dev().await;

    let mm = ModelManager::new().await?;

    // -- Define Routes
    let routes_hello = Router::new()
        .route("/hello", get(|| async { Html("Hello World") }))
        .route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .merge(routes_hello)
        .layer(middleware::from_fn(mw_response_map))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            mw_ctx_resolve,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - {addr}\n", "LISTENING");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
    // end region

    Ok(())
}