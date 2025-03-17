use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs; // FS: File system

const FILE_NAME: &str = "history.csv";

#[derive(Debug)]
struct HistoryPoint {
    point_type: String,
    tag: String,
    text: String,
    life: i32,
}

impl HistoryPoint {
    fn new(row: StringRecord) -> HistoryPoint {
        let life = row.get(3).unwrap().trim();
        let life: i32 = life.parse().unwrap_or(0);

        return HistoryPoint {
            point_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life,
        };
    }
}

fn main() {
    let mut history: HashMap<String, HistoryPoint> = HashMap::new();

    let content = fs::read_to_string(FILE_NAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let res = result.unwrap();
        let history_point = HistoryPoint::new(res);
        history.insert(history_point.tag.clone(), history_point);
    }

    println!("{:?}", history)
}
