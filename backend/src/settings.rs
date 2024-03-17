use std::env;

pub struct Settings {
    pub surreal_username: String,
    pub surreal_password: String,
    pub surreal_namespace: String,
    pub surreal_database: String,
}

impl Settings {
    pub fn from_env() -> Self {
        Self {
            surreal_username: env::var("SURREAL_USERNAME").unwrap_or("root".into()),
            surreal_password: env::var("SURREAL_PASSWORD").unwrap_or("root".into()),
            surreal_namespace: env::var("SURREAL_NAMESPACE").unwrap_or("io.github.pandier".into()),
            surreal_database: env::var("SURREAL_DATABASE").unwrap_or("surrealtasks".into()),
        }
    }
}
