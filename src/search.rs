use bincode;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

pub enum Search {
    Input(Input),
}

impl Search {
    pub fn search_for(word: &str) -> String {
        let mut current_dir: std::path::PathBuf =
            env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("occurances.txt");
        let mut file = File::open(current_dir).expect("Failed to open file ");

        let mut data = vec![];

        file.read_to_end(&mut data)
            .expect("Failed to read contents of file");

        let decoded: HashMap<String, Vec<(String, usize)>> =
            bincode::deserialize(&data).expect("Failed to deserialize Bincode data");
        let ocurrance: Option<&Vec<(String, usize)>>;

        let filtered_list: HashMap<_, _> = decoded
            .iter()
            .filter(|dec_word| dec_word.0.contains(word))
            .collect();
        if filtered_list.contains_key(&word.to_string()) {
            ocurrance = filtered_list.get(&word.to_string()).copied();
        } else {
            return "No word found".to_string();
        }
        // if decoded.contains_key(word) {
        //     ocurrance = decoded.get(word);
        // } else {
        //     return "No word found".to_string();
        // }
        let mut res: Vec<(std::string::String, usize)> = ocurrance.unwrap().to_owned();
        res.sort_by(|a, b| b.1.cmp(&a.1));
        return res.first().unwrap().0.to_owned();
    }
}
type Input = String;
