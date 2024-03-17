use eyre::Result;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::Settings;

pub async fn init(settings: &Settings) -> Result<Surreal<Client>> {
    let database = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    database
        .signin(Root {
            username: &settings.surreal_username,
            password: &settings.surreal_password,
        })
        .await?;

    database
        .use_ns(&settings.surreal_namespace)
        .use_db(&settings.surreal_database)
        .await?;

    Ok(database)
}
