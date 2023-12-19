use reqwest::Response;
use serde::{Deserialize, Serialize};
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

pub async fn fetch_and_convert_into_vector(url: String) -> Result<Dmg, FetchError> {
    return match reqwest::get(url).await {
        Ok(res) => Ok(res.json::<Dmg>().await.unwrap()),
        Err(_) => Err(FetchError::FetchError),
    };
}
