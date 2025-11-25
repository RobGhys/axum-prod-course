#![allow(unused)]

use std::net::SocketAddr;
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolver;
use crate::web::mw_res_map::mw_response_map;
use crate::web::{routes_login, routes_static};
pub use self::error::{Error, Result};
mod ctx;
mod error;
mod log;
mod model;
mod web;
#[tokio::main]
async fn main() -> Result<()> {
    let mm = ModelManager::new().await?;

    // -- Define Routes

    let routes_all = Router::new()
        .merge(routes_login::routes())
        .layer(middleware::from_fn(mw_response_map))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {addr}\n");
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
    // end region

    Ok(())
}