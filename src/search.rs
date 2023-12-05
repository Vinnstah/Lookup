use std::fs::File;
use std::io::Read;
use bincode;

pub enum Search {
    Input(Input),

}

impl Search {
    pub fn get_document_contents(
        title: &str
    ) -> String {

        let mut file = File::open(title).expect("Failed to open file ");
        let mut data = vec![];

        file.read_to_end(&mut data).expect("Failed to read contents of file");

         bincode::deserialize(&data).expect("Failed to deserialize Bincode data")
     }
}
type Input = String;