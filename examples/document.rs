use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use reqwest::{blocking::Response, Client, Error};
use scraper::{Html, Selector};
use serde_json::{json, Map, Value};

fn main() -> std::io::Result<()> {
    // let url = env::var("TARGET_URL").expect("TELEGRAM_BOT_TOKEN not set");;
    // let res = reqwest::blocking::get(url).expect("Could not load url.");

    // let raw_html_string = res.text().unwrap();

    // let document = Html::parse_document(&raw_html_string);

    // let app_data_selector = Selector::parse("#__APP_DATA").unwrap();

    // let app_data_elem = document.select(&app_data_selector).next().unwrap();
    // let app_data = app_data_elem.text().next().unwrap();

    // let parsed: Value = serde_json::from_str(app_data).unwrap();

    // let title = &parsed["appState"]["loader"]["dataByRouteId"]["2a3f"]["catalogs"][0]["articles"]
    //     [0]["title"];

    // match File::open("latest_post_title.txt") {
    //     Ok(mut file) => {
    //         let mut contents = String::new();
    //         file.read_to_string(&mut contents).unwrap();

    //         if contents != title.to_string() {
    //             let mut file = File::create("latest_post_title.txt").unwrap();
    //             file.write_all(title.to_string().as_bytes())?;
    //         }

    //         // println!("{}", title);
    //     }
    //     Err(_) => {
    //         let mut file = File::create("latest_post_title.txt").unwrap();
    //         file.write_all(title.to_string().as_bytes())?;
    //     }
    // }

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id: i64 = env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID not set")
        .parse()
        .expect("faile to parse");

    send_message("Test from library".to_string(), &token, chat_id).expect("fail to send message");

    Ok(())

    // let appState = &appState["loader"];

    // println!("{}", parsed["appState"]);
    //     let first_div_elem = second_section_elem
    //         .select(&first_div_selector)
    //         .next()
    //         .unwrap();
    //     println!("{:?}", first_div_elem);
    // }

    // let post_selector = Selector::parse("div.css-1tl1y3y").unwrap();
    // let publish_date_selector = Selector::parse("a div h6").unwrap();

    // for node in second_section_elem.select(&post_selector) {
    //     println!("{:?}", node);
    //     let date_elem = node.select(&publish_date_selector).next().unwrap();
    //     let date = date_elem.text().collect::<String>();
    //     println!("{:?}", date);
    // }
    // let page_title = title_elem.text().collect::<String>();
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
