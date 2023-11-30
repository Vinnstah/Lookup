use std::{io::{stdin, stdout, Write}, ops::Add};
mod inverted_index;
use crate::inverted_index::ConvertToIndex;
use std::fs::File;

fn main() {
    let mut buffer = String::new();
    println!("What do you wish to search for?");
    stdin().read_line(&mut buffer).expect("Unable to read string");

    ConvertToIndex::save(&buffer, "firstDoc");

    //Convert input buffer to hashset to remove duplicates
    let converted_str: std::collections::HashSet<String> = ConvertToIndex::convert(&buffer);

    let mut file = File::create("test.txt").unwrap();
    //let buffered_string = converted_str.into_iter().map(|x| x.as_bytes()).collect();

    //Convert the hashset back to string to store it. Should use serialize instead with bincode or smth else
    let x: String = converted_str.iter().map(|x| x.to_owned().to_string().add(" ")).collect();
    let convert_to_string_from_set = String::from_iter(converted_str.into_iter());
    
    
    file.write_all(x.as_bytes());
    println!("{:#?}", ConvertToIndex::count_occurances(&buffer, ConvertToIndex::convert(&buffer)));
    // println!("{}", ConvertToIndex::get_docucoument_contents("firstDoc.txt"))

}
