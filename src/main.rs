use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use dotenv::dotenv;
use scraper::{Html, Selector};
use serde_json::{json, Map, Value};

fn main() -> std::io::Result<()> {
    dotenv().expect("can not find env file");

    // read env variables
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("failed to read TELEGRAM_BOT_TOKEN");
    let chat_id: i64 = env::var("TELEGRAM_CHAT_ID")
        .expect("failed to read TELEGRAM_CHAT_ID")
        .parse()
        .expect("failed to parse");
    let url = env::var("TARGET_URL").expect("failed to read TARGET_URL");

    let res = reqwest::blocking::get(&url).expect("Could not load url.");

    let raw_html_string = res.text().unwrap();

    let document = Html::parse_document(&raw_html_string);

    let app_data_selector = Selector::parse("#__APP_DATA").unwrap();

    let app_data_elem = document.select(&app_data_selector).next().unwrap();
    let app_data = app_data_elem.text().next().unwrap();

    let parsed: Value = serde_json::from_str(app_data).unwrap();

    let title = &parsed["appState"]["loader"]["dataByRouteId"]["2a3f"]["catalogs"][0]["articles"]
        [0]["title"];

    match File::open("latest_post_title.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            if contents != title.to_string() {
                let mut file = File::create("latest_post_title.txt").unwrap();
                file.write_all(title.to_string().as_bytes())?;

                // send message by Telegram
                let msg = format!("New post {} has released. Check out {}", title, url);
                send_message(msg, &token, chat_id).expect("fail to send message");
            }
        }
        Err(_) => {
            let mut file = File::create("latest_post_title.txt").unwrap();
            file.write_all(title.to_string().as_bytes())?;
        }
    }

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
