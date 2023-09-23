use std::fs::File;

use scraper::{Html, Selector};

fn main() {
    let url =
        "https://www.binance.com/en/support/announcement/new-cryptocurrency-listing?c=48&navld=48";
    let res = reqwest::blocking::get(url).expect("Could not load url.");


    let raw_html_string = res.text().unwrap();

    let document = Html::parse_document(&raw_html_string);

    let page_title = Selector::parse("h1.css-dwc418").unwrap();
    // let article_name = Selector::parse("a div").unwrap();
    let href = Selector::parse("a.css-1ey6mep").unwrap();

    let section_selector = Selector::parse("section.css-14d7djd").unwrap();
    let first_div_selector = Selector::parse("div.css-148156o").unwrap();

    for second_section_elem in document.select(&section_selector) {
        let first_div_elem = second_section_elem
            .select(&first_div_selector)
            .next()
            .unwrap();
        println!("{:?}", first_div_elem);
    }

    let post_selector = Selector::parse("div.css-1tl1y3y").unwrap();
    let publish_date_selector = Selector::parse("a div h6").unwrap();

    // for node in second_section_elem.select(&post_selector) {
    //     println!("{:?}", node);
    //     let date_elem = node.select(&publish_date_selector).next().unwrap();
    //     let date = date_elem.text().collect::<String>();
    //     println!("{:?}", date);
    // }
    // let page_title = title_elem.text().collect::<String>();
}
