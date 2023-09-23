use std::env;

use serde_json::{json, Value, Map};

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id: i64 = env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID not set")
        .parse()
        .expect("faile to parse");

    send_message("Test from library".to_string(), &token, chat_id).expect("fail to send message");
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
