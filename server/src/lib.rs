pub mod entities {
    pub mod disk_image;
    pub mod user;
}

pub mod repositories {
    pub mod postgres;
    pub mod user;
}

pub(crate) mod usecases {
    pub mod dmg;
    pub mod user;
}

pub mod application {
    pub mod dmg;
    pub mod user;
}

pub(crate) mod error {
    pub mod db_error;
}
pub mod consts {
    pub const BASE_PATH: &str = "./assets/";
    pub const TEMPLATE_DMG_PATH: &str = "./assets/demo.dmg";

    pub fn db_url() -> String {
        dotenvy::dotenv().ok();
        std::env::var("DATABASE_URL").unwrap()
    }
}
