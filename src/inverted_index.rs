use crate::stop_words;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

pub enum ConvertToIndex {}

impl ConvertToIndex {
    pub fn count_occurances(content: &String, keys: HashSet<String>) -> HashMap<String, usize> {
        let mut occurance_map: HashMap<String, usize> = HashMap::new();

        for key in keys {
            if stop_words::STOP_WORDS.contains(&&key.as_str()) || key.len() == 1 {
                continue;
            }
            let occurances_in_content = content.to_lowercase().matches(&key).count();
            occurance_map.insert(key, occurances_in_content);
        }

        return occurance_map;
    }

    pub fn convert(input: &String) -> HashSet<String> {
        input
            .split_whitespace()
            .into_iter()
            .map(|word| word.to_lowercase().to_string())
            .collect::<HashSet<_>>()
    }

    pub fn save(input: &String, title: &str) -> Result<(), Error> {
        let mut current_dir: std::path::PathBuf =
            env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("data");

        if !current_dir.as_path().exists() {
            fs::create_dir("data")?;
        }
        current_dir.push(title.to_string() + ".txt");
        let mut file = File::create(current_dir).expect("Failed to create file");
        let encoded_content = bincode::serialize(input).expect("Failed to serialize bincode");

        file.write_all(&encoded_content)
    }

    pub fn read_occurances() -> HashMap<String, Vec<(String, usize)>> {
        let mut current_dir: std::path::PathBuf =
            env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("occurances.txt");
        let mut file = File::open(current_dir).expect("Failed to open file ");
        let mut data = vec![];

        file.read_to_end(&mut data)
            .expect("Failed to read contents of file");

        let res = bincode::deserialize(&data);

        match res {
            Ok(result) => result,
            Err(_) => HashMap::new(),
        }
    }

    pub fn save_occurances(data: HashMap<String, Vec<(String, usize)>>) -> Result<(), Error> {
        let mut current_dir: std::path::PathBuf =
            env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("occurances.txt");
        let mut file = File::create(current_dir).expect("Failed to open file ");

        let encoded_content = bincode::serialize(&data).expect("Failed to serialize bincode");
        file.write_all(&encoded_content)
    }

    pub fn handle_occurances(input: HashMap<String, usize>, url: &str) -> Result<(), Error> {
        let mut current_dir: std::path::PathBuf =
            env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("data");

        if !current_dir.as_path().exists() {
            fs::create_dir("data")?;
        }

        let mut occurances = ConvertToIndex::read_occurances();

        input.iter().for_each(|occurance| {
            if occurances.contains_key(occurance.0) {
                occurances
                    .get_mut(occurance.0)
                    .unwrap()
                    .push((url.to_string(), *occurance.1));
            } else {
                occurances.insert(
                    occurance.0.to_string(),
                    vec![(url.to_string(), *occurance.1)],
                );
            }
        });

        ConvertToIndex::save_occurances(occurances).expect("Failed to save occurances to file");

        Ok(())
    }
}
