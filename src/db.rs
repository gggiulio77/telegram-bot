use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use teloxide::types::ChatId;

use std::error::Error;

#[derive(Debug, Serialize)]
pub struct User<'a> {
    pub chat_id: &'a ChatId,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn init() -> Result<Surreal<Client>, Box<dyn Error>> {
    let db = Surreal::new::<Ws>(std::env::var("DB_URL")?).await?;

    db.signin(Root {
        username: &std::env::var("DB_USER")?,
        password: &std::env::var("DB_PASSWORD")?,
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    Ok(db)
}
