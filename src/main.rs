mod handler;

use crate::handler::message_handler;
use std::error::Error;

use dotenv::dotenv;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use teloxide::{
    error_handlers::IgnoringErrorHandlerSafe, prelude::*, update_listeners::webhooks, Bot,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");

    let db = Surreal::new::<Ws>(std::env::var("DB_URL")?).await?;

    db.signin(Root {
        username: &std::env::var("DB_USER")?,
        password: &std::env::var("DB_PASSWORD")?,
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    let bot = Bot::from_env();

    let addr = ([127, 0, 0, 1], 8443).into();
    let url = "https://telegram-bot.default.sentateenesta.duckdns.org"
        .parse()
        .unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, IgnoringErrorHandlerSafe::new())
        .await;

    Ok(())
}

// TODO: add production webhook support
// TODO: re estructure project
