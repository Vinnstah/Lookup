use clap::Parser;
use lookup::search::Search;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    search_word: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let search_result_url = Search::search_for(&args.search_word);
    let mut list_dir = Command::new("open");

    list_dir
        .arg(search_result_url)
        .spawn()
        .expect("process failed to execute");

    Ok(())
}
