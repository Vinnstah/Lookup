use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Read;
use bincode;
use std::env;
use std::collections::HashMap;

pub enum Search {
    Input(Input),

}

impl Search {
    pub fn search_for(
        word: &str
    ) -> String {

        let mut current_dir: std::path::PathBuf = env::current_dir().expect("Couldn't work out the current directory");
        current_dir.push("occurances.txt");
        let mut file = File::open(current_dir).expect("Failed to open file ");

        let mut data = vec![];

        file.read_to_end(&mut data).expect("Failed to read contents of file");

         let decoded: HashMap<String, Vec<(String, usize)>> = bincode::deserialize(&data).expect("Failed to deserialize Bincode data");
         let mut ocurrance: Option<&Vec<(String, usize)>>;
         if decoded.contains_key(word) {
             ocurrance = decoded.get(word);
            } else  {
                return "No word found".to_string()
            }
            let mut res: Vec<(std::string::String, usize)> = ocurrance.unwrap().to_owned();
            res.sort_by(|a, b| b.1.cmp(&a.1)); 
            // .find(predicate) .max_by(|x, y|y.1.cmp(&x.1));
            println!("{:#?}", res);
        return res.first().unwrap().0.to_owned();
    //     .sort_by(|a, b| {
    //         let borrow = a.1.clone();
    //         b.1.cmp(borrow.borrow_mut()) 
    // });
     }
}
type Input = String;