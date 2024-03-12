use eyre::Result;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn init() -> Result<Surreal<Client>> {
    let database = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    database
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

    database
        .use_ns("io.github.pandier")
        .use_db("surrealtasks")
        .await?;

    Ok(database)
}
