use std::{collections::{HashMap, HashSet}, io::Error};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use crate::stop_words;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize)]
struct InvertedIndex {
    occurance: HashMap<String, u32>,
    words: HashMap<String, u32>
}

pub enum Crawler {
    Succes(ConvertToIndex)
}

pub enum ConvertToIndex {
    
}

impl ConvertToIndex {
    pub fn count_occurances(content: &String, keys: HashSet<String>) -> HashMap<String, usize> {
        let mut occurance_map: HashMap<String, usize> = HashMap::new();

        for key in keys {
            if stop_words::STOP_WORDS.contains(&&key.as_str()) || key.len() == 1 { continue; }
            let occurances_in_content = content.to_lowercase().matches(&key).count();
            occurance_map.insert(key, occurances_in_content);
        }


        return occurance_map
    }

    pub fn convert(input: &String) -> HashSet<String> {
        input
        .split_whitespace()
        .into_iter()
        .map(|word| word.to_lowercase().to_string())
        .collect::<HashSet<_>>()
    }

    pub fn save(
        input: &String, 
        title: &str
    ) -> Result<(), Error> {
        let mut current_dir: std::path::PathBuf = env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("data");

        if !current_dir.as_path().exists() {
            fs::create_dir("data")?;
        }

        current_dir.push(title.to_string() + ".txt");
        let result = File::create(current_dir);
        let encoded_content = bincode::serialize(input).expect("Failed to serialize bincode");
        match result {
            Ok(mut file) => file.write_all(&encoded_content),
            Err(err) => Err(err)
        }
    }


}




