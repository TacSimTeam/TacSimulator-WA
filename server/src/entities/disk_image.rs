use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dmg {
    name: String,
    data: Vec<u8>,
}

impl Dmg {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Dmg { name, data }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
