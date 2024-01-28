use axum::Router;
use server::application::dmg::load_dmg_router;
use server::application::user::load_user_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/user", load_user_router().await)
        .nest("/dmg", load_dmg_router())
        .layer(CorsLayer::very_permissive());
    let port = std::env::var("PORT").unwrap();
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
