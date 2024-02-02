use crate::consts::{BASE_PATH, TEMPLATE_DMG_PATH};
use crate::entities::disk_image::Dmg;
use anyhow::Result;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use std::fs::File;
use std::io::{Read, Write};

fn get_dmg_data(path: String) -> Dmg {
    let dmg_path = BASE_PATH.to_owned() + &*path + ".dmg";
    let mut dmg = File::open(dmg_path.clone()).unwrap();
    let mut buf = Vec::new();
    dmg.read_to_end(&mut buf).unwrap();
    Dmg::new(dmg_path, buf)
}

pub async fn get_dmg(Path(path): Path<String>) -> impl IntoResponse {
    Json(get_dmg_data(path))
}

fn update_dmg_data(dmg: Dmg) -> Result<()> {
    let dmg_path = BASE_PATH.to_owned() + &*dmg.get_name() + ".dmg";
    let mut fd = File::create(dmg_path.clone())?;
    fd.write_all(dmg.get_data())?;
    Ok(())
}

pub async fn update_dmg(Json(dmg): Json<Dmg>) -> impl IntoResponse {
    match update_dmg_data(dmg) {
        Ok(_) => (StatusCode::OK, "success".to_string()).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "fs error".to_string()).into_response(),
    }
}

pub fn create_new_dmg(name: String) -> std::io::Result<()> {
    std::fs::copy(TEMPLATE_DMG_PATH, &format!("{}{}.dmg", BASE_PATH, name)).unwrap();
    Ok(())
}
