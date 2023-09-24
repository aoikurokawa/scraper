use std::env;

use dotenv::dotenv;
use serde_json::{json, Map, Value};
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().expect("can not find env file");
    pretty_env_logger::init();
    log::info!("Starting command bot..");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age..", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };

    Ok(())
}

fn send_message(
    msg: String,
    token: &str,
    chat_id: i64,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), json!(chat_id));
    request_body.insert(
        "parse_mode".to_string(),
        Value::String("MarkdownV2".to_string()),
    );

    let url = format!(
        "https://api.telegram.org/bot{token}/sendMessage",
        token = &token
    );
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).json(&json!(request_body)).send()?;

    Ok(resp)
}
