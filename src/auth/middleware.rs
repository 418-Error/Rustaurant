use axum::{extract::Request, middleware::Next, response::Response};

use crate::auth::auth::verify_token;

pub async fn auth_middleware(
    request: Request,
    next: Next
) -> Response {
    println!("Auth middleware");
    let token = match request.headers().get(http::header::AUTHORIZATION) {
        Some(token) => {
            // Remove the "Bearer " prefix
            let token = token.to_str().unwrap().split_whitespace().last().unwrap();
            token
        },
        None => {
            return Response::builder()
                .status(401)
                .body("Unauthorized".into())
                .unwrap();
        }
    };

    if verify_token(token).is_err() {
        return Response::builder()
            .status(401)
            .body("Unauthorized".into())
            .unwrap();
    }

    let response = next.run(request).await;
    response
}
