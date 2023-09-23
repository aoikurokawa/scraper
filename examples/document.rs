use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use scraper::{Html, Selector};
use serde_json::Value;

fn main() -> std::io::Result<()> {
    let url = env::var("TARGET_URL").expect("TELEGRAM_BOT_TOKEN not set");
    let res = reqwest::blocking::get(url).expect("Could not load url.");

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
            }

            // println!("{}", title);
        }
        Err(_) => {
            let mut file = File::create("latest_post_title.txt").unwrap();
            file.write_all(title.to_string().as_bytes())?;
        }
    }

    Ok(())
}
