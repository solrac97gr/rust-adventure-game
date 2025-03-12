use csv::{ReaderBuilder, StringRecord};
use std::fs; // FS: File system

const FILE_NAME: &str = "history.csv";

fn main() {
    let content = fs::read_to_string(FILE_NAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        println!("{}", result.unwrap().get(2).unwrap().to_string());
    }
}
