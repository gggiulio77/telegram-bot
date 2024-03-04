use std::{convert::Infallible, error::Error};

use axum::{routing, Router};
use surrealdb::{engine::remote::ws::Client, Surreal};
use teloxide::{
    dispatching::DpHandlerDescription, prelude::*, types::Me, update_listeners::webhooks,
    update_listeners::UpdateListener, utils::command::BotCommands, Bot,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::db::{Record, User};

type BotHandler =
    Handler<'static, DependencyMap, Result<(), Box<dyn Error + Send + Sync>>, DpHandlerDescription>;

#[derive(BotCommands)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    db: Surreal<Client>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                // Just send the description of all commands.
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                let data = User {
                    chat_id: &msg.chat.id,
                };
                // TODO: move this to a repository
                // TODO: if first_name si None, it means the bot is in a public chat. Add logic to manage this case.
                // TODO: refactor to use an email instead of first_name. If we cant get it from telegram User, add a step to require it.
                let created: Vec<Record> = if let Some(first_name) = msg.chat.first_name() {
                    db.update(("user", first_name))
                        .content(data)
                        .await?
                        .unwrap()
                } else {
                    db.create("user").content(data).await?
                };

                dbg!(created);

                bot.send_message(msg.chat.id, "Welcome!").await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

pub async fn init_with_webhook(
) -> Result<(Bot, BotHandler, impl UpdateListener<Err = Infallible>), Box<dyn Error>> {
    let bot = Bot::from_env();

    let port: u16 = match std::env::var("PORT") {
        Ok(port) => port.parse().unwrap(),
        Err(_) => 8080,
    };
    let address = format!("127.0.0.1:{}", port).parse()?;
    let url = std::env::var("TELOXIDE_URL")?.parse().unwrap();

    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    let options = webhooks::Options::new(address, url);
    let (mut update_listener, stop_flag, app) =
        webhooks::axum_to_router(bot.clone(), options).await?;
    let stop_token = update_listener.stop_token();

    tokio::spawn(async move {
        let new_router = Router::from(app)
            .route("/", routing::get(|| async { "Ok" }))
            .route("/health/*key", routing::get(|| async { "Ok" }))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            );

        axum::Server::bind(&address)
            .serve(new_router.into_make_service())
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|err| {
                stop_token.stop();
                err
            })
            .expect("Axum server error");
    });

    Ok((bot, handler, update_listener))
}
