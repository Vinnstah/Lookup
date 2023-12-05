use std::{io::{stdin, stdout, Write}, ops::Add};
use lookup::{inverted_index::ConvertToIndex, search::Search};
use lookup::scraper::RequestClient;
use lookup::search;
use std::fs::File;
use tokio::{self, time::Sleep};
use reqwest::Url;
use std::process::Command;
use tokio::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = RequestClient::new();

    println!("{:#?}", Search::search_for("test"));
    // SAVE THIS CODE
    // let mut base_url = "";
    // let mut res = client.scrape("https://doc.rust-lang.org/rust-by-example/").await;
    // loop {
    //     println!("Result {:#?}", res);
    //     let base_url = &res.unwrap();
    //     res = client.scrape(base_url).await;
    // }


    
    // let mut list_dir = Command::new("open");
    // list_dir.arg("https://doc.rust-lang.org/rust-by-example/hello.html").spawn().expect("process failed to execute");

// Execute `ls` in the current directory of the program.
    // Command::new("iterm2")
    // .arg("open https://doc.rust-lang.org/rust-by-example/hello.html")
    // .spawn()
    // .expect("failed to execute process");

//     let config = CrawlerConfig::default().allow_domain("doc.rust-lang.org");
// let mut collector = Collector::new(RustLang::default(), config);

// collector.crawler_mut().visit_with_state(
//     "https://doc.rust-lang.org/rust-by-example/hello.html",
// /    RustLangState::Content { text: "HO".to_string() }
// );

// while let Some(output) = collector.next().await {
//     let post = output?;
//     dbg!(post);
// }
Ok(())


    // let mut buffer = String::new();
    // println!("What do you wish to search for?");
    // stdin().read_line(&mut buffer).expect("Unable to read string");

    // ConvertToIndex::save(&buffer, "firstDoc").expect("Failed to save to file");

    // //Convert input buffer to hashset to remove duplicates
    // let converted_str: std::collections::HashSet<String> = ConvertToIndex::convert(&buffer);

    // let mut file = File::create("test.txt").unwrap();
    // //let buffered_string = converted_str.into_iter().map(|x| x.as_bytes()).collect();

    // //Convert the hashset back to string to store it. Should use serialize instead with bincode or smth else
    // let x: String = converted_str.iter().map(|x| x.to_owned().to_string().add(" ")).collect();
    // let _convert_to_string_from_set = String::from_iter(converted_str.into_iter());
    
    
    // file.write_all(x.as_bytes()).expect("Failed to write to file");
    // println!("{:#?}", ConvertToIndex::count_occurances(&buffer, ConvertToIndex::convert(&buffer)));
    // Ok(())
    // // println!("{}", ConvertToIndex::get_docucoument_contents("firstDoc.txt"))

}
