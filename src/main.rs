use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use dotenv::dotenv;
use scraper::{Html, Selector};
use serde_json::Value;
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
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "handle getting new post.")]
    NewPost,
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
        Command::NewPost => {
            let text = get_latest_post().await?;

            bot.send_message(msg.chat.id, text).await?
        }
    };

    Ok(())
}

async fn get_latest_post() -> reqwest::Result<String> {
    let url = env::var("TARGET_URL").expect("failed to read TARGET_URL");

    let res = reqwest::get(&url).await?;

    let raw_html_string = res.text().await?;

    let document = Html::parse_document(&raw_html_string);

    let app_data_selector = Selector::parse("#__APP_DATA").unwrap();

    let app_data_elem = document.select(&app_data_selector).next().unwrap();
    let app_data = app_data_elem.text().next().unwrap();

    let parsed: Value = serde_json::from_str(app_data).unwrap();

    let title = &parsed["appState"]["loader"]["dataByRouteId"]["2a3f"]["catalogs"][0]["articles"]
        [0]["title"];

    let mut res = String::new();
    match File::open("latest_post_title.txt") {
        Ok(mut file) => {
            println!("reading file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            println!("contents is {}, title is {}", contents, title);
            if contents != title.to_string() {
                let mut file = File::create("latest_post_title.txt").unwrap();
                file.write_all(title.to_string().as_bytes()).expect("");

                println!("send message");
                // send message by Telegram
                res = format!("New post {} has released. Check out {}", title, url);
            }
        }
        Err(_) => {
            println!("creating file");
            let mut file = File::create("latest_post_title.txt").unwrap();
            file.write_all(title.to_string().as_bytes()).expect("");
        }
    }

    res = if res.is_empty() {
        format!("not posted yet")
    } else {
        res
    };

    Ok(res)
}
