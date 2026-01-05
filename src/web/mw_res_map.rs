use std::sync::Arc;
use axum::extract::Request;
use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::middleware::Next;
use serde_json::json;
use uuid::Uuid;

pub async fn mw_response_map(
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let ctx = req.extensions().get::<Ctx>().cloned();

    println!("->> {:<12} - mw_response_map", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let res = next.run(req).await;
    // -- Get the eventual response error.
    let web_error = res.extensions().get::<Arc<web::Error>>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response.
    let error_response =
        client_status_error
            .as_ref()
            .map(|(status_code, client_error)| {
                let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

                println!("->> CLIENT ERROR BODY:\n{client_error_body}");

                // Build the new response from the client_error_body
                (*status_code, Json(client_error_body)).into_response()
            });

    // -- Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(
        uuid, method, uri, ctx, web_error.map(|e| e.as_ref()), client_error
    ).await;

    println!("\n");

    error_response.unwrap_or(res)
}
