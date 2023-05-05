mod bot;
mod db;

use dotenv::dotenv;
use std::error::Error;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    log::info!("Conencting db...");

    let db = db::init().await?;

    log::info!("Starting bot...");

    let (bot, handler, update_listener) = bot::init_with_webhook().await?;

    // TODO: think a way to support polling and webhook
    // TODO: add error handler

    Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![db])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(update_listener, LoggingErrorHandler::new())
        .await;

    Ok(())
}
