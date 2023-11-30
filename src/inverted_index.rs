use std::{collections::{HashMap, HashSet}, io::Error};
use serde::{Serialize, Deserialize};
use std::fs::File;
use bincode::serialize;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize)]
struct InvertedIndex {
    occurance: HashMap<String, u32>,
    words: HashMap<String, u32>
}

pub struct ConvertToIndex {

}

impl ConvertToIndex {
    //, word_map: HashMap<String, u32>
    pub fn convert(input: &String) -> HashSet<String> {
        
        input
        .split(' ')
        .into_iter()
        .map(|word| word.to_string())
        .collect::<HashSet<_>>()
    }

    pub fn save(input: &String, title: &str) -> Result<(), Error> {
        let result = File::create(title.to_string() + ".txt");
        let encoded_content = bincode::serialize(input).expect("Failed to serialize bincode");
        match result {
            Ok(mut file) => file.write_all(&encoded_content),
            Err(err) => Err(err)
        }
    }

    pub fn get_document_contents(title: &str) -> String {
       let mut file = File::open(title).expect("Failed to open file ");
       let mut data = vec![];
       file.read_to_end(&mut data).expect("Failed to read contents of file");
        bincode::deserialize(&data).unwrap()
    }
}