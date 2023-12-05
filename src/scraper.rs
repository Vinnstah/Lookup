// use futures::StreamExt;
// use reqwest::Url;
// use voyager::scraper::Selector;
// use voyager::{Collector, Crawler, CrawlerConfig, Response, Scraper};
// use anyhow::Result;

// pub struct RustLang {
//     title_selector: Selector,
//     body_selector: Selector,
//     base_url: Url
// }

// impl Default for RustLang {
//     fn default() -> Self {
//         Self {
//             title_selector: Selector::parse("a.header").unwrap(),
//             // body_selector: Selector::parse("div#content main").unwrap(),
//             body_selector: Selector::parse("#content > main").unwrap(),
//             base_url: Url::parse("https://doc.rust-lang.org/rust-by-example/").unwrap(),
//         }
//     }
// }

// #[derive(Debug)]
// enum MainContent {
//     Paragraph(Selector),
//     Code(Selector),
//     H3(Selector),
//     BlockQuote(Selector)
// }

// #[derive(Debug)]
// pub enum RustLangState {
//     Content(Content),
// }

// #[derive(Debug)]
// pub struct Content {
//     text: String
// }

// impl Scraper for RustLang {
//     type Output = Content;
//     type State = RustLangState;

//     fn scrape(
//         &mut self,
//         response: Response<Self::State>,
//         crawler: &mut Crawler<Self>,
//     ) -> Result<Option<Self::Output>> {
//         let html = response.html();

//         if let Some(state) = response.state {
//             match state {
//                 RustLangState::Content { .. } => {
//                     for (idx, el) in html.select(&self.body_selector).enumerate() {
//                         let val = el.value();

//                         let title = html
//                         .select(&self.title_selector)
//                         .map(|el| el.inner_html())
//                         .next();

//                         println!("{:#?}", title);
//                         let entry_id = val.attr("main");

//                         let content = Content {
//                             text: String::from(el.inner_html())
//                         };

//                         crawler.visit_with_state(self.base_url.clone(), state);
//                         return Ok(Some(content));
//                     }
//                 }
//             }
//         }
//         Ok(None)
//     }
    
// }

use std::{ops::Deref};
use futures::{TryFutureExt, FutureExt};
use reqwest::{self, Url, Client};
use scraper::{self, Selector, Html};
use async_recursion::async_recursion;

pub struct RustLang {
    title_selector: Selector,
    body_selector: Selector,
    next_selector: Selector,
    base_url: Url
}

impl Default for RustLang {
    fn default() -> Self {
        Self {
            title_selector: Selector::parse("a.header").unwrap(),
            body_selector: Selector::parse("#content > main ").unwrap(),
            next_selector: Selector::parse("#content > nav > a.mobile-nav-chapters.next").unwrap(),
            base_url: Url::parse("https://doc.rust-lang.org/rust-by-example/").unwrap(),
        }
    }
}
#[derive(Debug)]
pub struct ScrapeResponse {
    pub title: String,
    pub body: String,
    pub next_url: String
}

unsafe impl Send for ScrapeResponse {}
unsafe impl Send for RequestClient {}
unsafe impl Sync for ScrapeResponse {}

impl ScrapeResponse {
    pub fn new()-> Self {
        Self {
            title: String::new(),
            body: String::new(), 
            next_url: String::new()
        }

    }
}

pub struct RequestClient {
    client: Client
}

impl RequestClient {
    pub fn new() -> Self {
       RequestClient { client: Client::new() }
    }
   
    pub async fn scrape(&self, url: &str) -> Option<String>  {
        let res = self.client.get(url).send().await.expect("No response fro URL");
        let body = res.text().await.expect("No body found");
        let document = Html::parse_document(&body);

        let mut scrape_response: ScrapeResponse = ScrapeResponse::new();

        let mut title: Vec<&str> = vec![];

        document.select(&RustLang::default().title_selector).by_ref().for_each(|temp_title| {
            let temp_title_list: Vec<&str> = temp_title.text().collect();
            title.push(temp_title_list[0]);
        });

        scrape_response.title = title.first().unwrap_or(&"default").to_string();
        let mut body_text: Vec<&str> = vec![];

        document.select(&RustLang::default().body_selector).for_each(|element| {
            let mut temp_body_list: Vec<&str> = element.text().collect();
            body_text.append(&mut temp_body_list);
        });

        scrape_response.body = body_text.join(" ");
        for link in document.select(&RustLang::default().next_selector) {
            if let Some(href) = link.value().attr("href") {
                println!("href: {}", href);
                scrape_response.next_url = href.to_string();
            } else {
                println!("No more pages, ending loop");
                return None
            }
        }
        let mut temp_removable_stirng = String::new();
        for (char) in scrape_response.next_url.clone().chars() {
            println!(" char {}", char);
            if char == '.' {
                temp_removable_stirng.push(char);
            } else if char == '/' {
                temp_removable_stirng.push(char);
            } else { break }
        }
        scrape_response.next_url = scrape_response.next_url.strip_prefix(&temp_removable_stirng).expect("Unable to strip prefix").to_string();
        println!("NExt URL: {:#?}", scrape_response.next_url);
        let url = "https://doc.rust-lang.org/rust-by-example/".to_owned() + &scrape_response.next_url;
        return Some(url)
    }
}