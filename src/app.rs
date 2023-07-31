use axum::{ 
    Router,
    routing,
    response,
    http::{StatusCode, request::Parts},
    extract::FromRequestParts,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
struct RequestData {
    route: String
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestData
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(Self { route: parts.uri.path().to_string() })
    }
}

async fn hello_word() -> String {
    return String::from("Hello, world!");
}

async fn handler_not_found(req: RequestData) -> impl response::IntoResponse {
    return (StatusCode::NOT_FOUND, format!("Cannot get {}", req.route))
}

pub async fn create_app() -> Router {
    let router = Router::new().route("/", routing::get(hello_word));

    let router = router.fallback(handler_not_found);

    return router;
}