use crate::inverted_index::ConvertToIndex;
use reqwest::{self, Client};
use scraper::{self, Html, Selector};
pub struct RustLang {
    title_selector: Selector,
    body_selector: Selector,
    next_selector: Selector,
}

impl Default for RustLang {
    fn default() -> Self {
        Self {
            title_selector: Selector::parse("a.header").unwrap(),
            body_selector: Selector::parse("#content > main ").unwrap(),
            next_selector: Selector::parse("#content > nav > a.mobile-nav-chapters.next").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ScrapeResponse {
    pub title: String,
    pub body: String,
    pub next_url: String,
}

unsafe impl Send for ScrapeResponse {}
unsafe impl Send for RequestClient {}
unsafe impl Sync for ScrapeResponse {}

impl ScrapeResponse {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            body: String::new(),
            next_url: String::new(),
        }
    }
}

pub struct RequestClient {
    client: Client,
}

impl RequestClient {
    pub fn new() -> Self {
        RequestClient {
            client: Client::new(),
        }
    }

    pub async fn scrape(&self, url: &str) -> Option<String> {
        let res = self
            .client
            .get(url)
            .send()
            .await
            .expect("No response fro URL");
        let body = res.text().await.expect("No body found");
        let document = Html::parse_document(&body);

        let mut scrape_response: ScrapeResponse = ScrapeResponse::new();

        let mut title: Vec<&str> = vec![];

        document
            .select(&RustLang::default().title_selector)
            .for_each(|temp_title| {
                let temp_title_list: Vec<&str> = temp_title.text().collect();
                title.push(temp_title_list[0]);
            });

        scrape_response.title = title
            .first()
            .unwrap_or(&"default")
            .to_string()
            .replace("/", "-");
        let mut body_text: Vec<&str> = vec![];

        document
            .select(&RustLang::default().body_selector)
            .for_each(|element| {
                let mut temp_body_list: Vec<&str> = element.text().collect();
                body_text.append(&mut temp_body_list);
            });

        scrape_response.body = body_text.join(" ");
        for link in document.select(&RustLang::default().next_selector) {
            if let Some(href) = link.value().attr("href") {
                scrape_response.next_url = href.to_string();
            } else {
                println!("No more pages, ending loop");
                return None;
            }
        }

        let hash_set = ConvertToIndex::convert(&scrape_response.body);
        let occurances = ConvertToIndex::count_occurances(&scrape_response.body, hash_set);

        ConvertToIndex::handle_occurances(occurances, &url)
            .expect("Failed to save occurances to file");

        println!("title {}", &scrape_response.title);
        ConvertToIndex::save(&scrape_response.body, &scrape_response.title)
            .expect("Failed to save scraped site");

        let mut prefix = String::new();
        for char in scrape_response.next_url.clone().chars() {
            if char == '.' || char == '/' {
                prefix.push(char);
            } else {
                break;
            }
        }
        scrape_response.next_url = scrape_response
            .next_url
            .strip_prefix(&prefix)
            .expect("Unable to strip prefix")
            .to_string();
        let url =
            "https://doc.rust-lang.org/rust-by-example/".to_owned() + &scrape_response.next_url;
        return Some(url);
    }
}

// SAVE THIS CODE
// let mut base_url = "";
// let mut res = client.scrape("https://doc.rust-lang.org/rust-by-example/").await;
// loop {
//     println!("Result {:#?}", res);
//     let base_url = &res.unwrap();
//     res = client.scrape(base_url).await;
//     if base_url == "https://doc.rust-lang.org/rust-by-example/meta/playground.html" || base_url == "" {
//         break;
//     }
// }
