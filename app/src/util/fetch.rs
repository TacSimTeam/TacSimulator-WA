use app::consts::BASE_URL;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dmg {
    name: String,
    data: Vec<u8>,
}

impl Dmg {
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Clone, Debug)]
pub enum FetchError {
    FetchError,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FetchError")
    }
}

impl Error for FetchError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationResult {
    Success,
    CreateNewUser,
    PassWordIncorrect,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDmgName {
    pub r#type: ValidationResult,
    pub dmg_name: Option<String>,
}

pub async fn get_dmg_path(user_name: String, password: String) -> Result<String, FetchError> {
    let mut body = HashMap::new();
    body.insert("name", user_name);
    body.insert("password", password);
    let client = reqwest::Client::new();
    let res = client
        .post(BASE_URL.to_string() + "user/dmg_name")
        .json(&body)
        .send()
        .await
        .unwrap();
    if res.status() == StatusCode::OK || res.status() == StatusCode::CREATED {
        Ok(res.json::<UserDmgName>().await.unwrap().dmg_name.unwrap())
    } else {
        Err(FetchError::FetchError)
    }
}

pub async fn fetch_and_convert_into_vector(url: String) -> Result<Dmg, FetchError> {
    match reqwest::get(url).await {
        Ok(res) => Ok(res.json::<Dmg>().await.unwrap()),
        Err(_) => Err(FetchError::FetchError),
    }
}
