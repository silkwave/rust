use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use tracing::{error, info, warn};

pub async fn log_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();

    info!(
        target: "api_requests",
        "Request: {} {}",
        method,
        uri
    );
    info!(
        target: "api_requests",
        "Headers: {:?}",
        headers
    );

    let res = next.run(req).await;

    let status = res.status();
    if status.is_client_error() {
        warn!(
            target: "api_responses",
            "Client error response: {} {} -> {}",
            method,
            uri,
            status
        );
        // Here you could add more detailed logging for specific client errors
        if status == StatusCode::NOT_FOUND {
            error!(
                target: "api_responses",
                "404 Not Found: No route for {} {}",
                method,
                uri
            );
        }
    } else if status.is_server_error() {
        error!(
            target: "api_responses",
            "Server error response: {} {} -> {}",
            method,
            uri,
            status
        );
    } else {
        info!(
            target: "api_responses",
            "Successful response: {} {} -> {}",
            method,
            uri,
            status
        );
    }

    res
}
