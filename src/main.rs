use clap::Parser;
use lookup::{scraper::scraper::RequestClient, search::Search};
use std::process::Command;

#[derive(Parser)]
struct Cli {
    search_word: Option<String>,
    scrape: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if &args.search_word.clone().unwrap() == "scrape" {
        println!("HERE 1");
        let client = RequestClient::new();
        let _ = client.run_scrape().await;
    } else {
        let search_result_url = Search::search_for(&args.search_word.unwrap());
        let mut search = Command::new("open");
        search
            .arg(search_result_url)
            .spawn()
            .expect("process failed to execute");
    }


    Ok(())
}
