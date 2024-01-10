use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router, ServiceExt};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use server::consts::BASE_PATH;

#[derive(Serialize, Deserialize, Debug)]
struct Dmg {
    name: String,
    data: Vec<u8>,
}

fn get_dmg_data(path: String) -> Dmg {
    let dmg_path = BASE_PATH.to_owned() + &*path;
    println!("{}", dmg_path.clone());
    let mut dmg = std::fs::File::open(dmg_path).unwrap();
    let mut buf = Vec::new();
    dmg.read_to_end(&mut buf).unwrap();
    Dmg {
        name: path,
        data: buf,
    }
}

async fn get_dmg(Path(path): Path<String>) -> impl IntoResponse {
    Json(get_dmg_data(path))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/:path", get(get_dmg))
        .layer(CorsLayer::very_permissive());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
