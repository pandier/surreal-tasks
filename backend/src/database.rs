use eyre::Result;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth,
    Surreal,
};

use crate::Settings;

pub async fn init(settings: &Settings) -> Result<Surreal<Client>> {
    let database = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    database
        .signin(auth::Database {
            username: &settings.surreal_username,
            password: &settings.surreal_password,
            namespace: &settings.surreal_namespace,
            database: &settings.surreal_database,
        })
        .await?;

    Ok(database)
}
