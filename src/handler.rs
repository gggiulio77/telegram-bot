use std::error::Error;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use teloxide::{prelude::*, types::Me, utils::command::BotCommands, Bot};

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

#[derive(Debug, Serialize)]
struct User<'a> {
    chat_id: &'a ChatId,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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
                // TODO: save chat id
                let created: Record = db
                    .create("user")
                    .content(User {
                        chat_id: &msg.chat.id,
                    })
                    .await?;

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
