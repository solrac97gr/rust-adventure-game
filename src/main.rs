use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs; // FS: File system

const FILE_NAME: &str = "history.csv";
const START_TAG: &str = "INICIO";

#[derive(Debug)]
struct HistoryPoint {
    point_type: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<HistoryPoint>,
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
            options: Vec::new(),
        };
    }
}

fn main() {
    let mut life: i32 = 100;
    let mut current_tag: &str = START_TAG;
    let mut last_record: String = "".to_string();
    let mut history: HashMap<String, HistoryPoint> = HashMap::new();

    let content = fs::read_to_string(FILE_NAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let res = result.unwrap();
        let history_point = HistoryPoint::new(res);
        if history_point.point_type == "SITUACION" {
            let record_tag = history_point.tag.clone();
            history.insert(record_tag.clone(), history_point);
            last_record = record_tag
        } else if history_point.point_type == "OPCION" {
            if let Some(data) = history.get_mut(&last_record) {
                (*data).options.push(history_point);
            }
        }
    }

    // Game lopp
    loop {
        println!("You have {} life points", life);
        if let Some(data) = history.get(current_tag) {
            println!("{}", (*data).text);
            for (index, option) in data.options.iter().enumerate() {
                println!("[{}] {}", index, option.text);
            }
            println!("Select your option");
            let mut user_option_selection: String = "".to_string();
            std::io::stdin()
                .read_line(&mut user_option_selection)
                .unwrap();

            let user_option_selection = user_option_selection.trim().parse().unwrap_or(99);
            if let Some(selection) = &data.options.get(user_option_selection) {
                current_tag = selection.tag.as_str();
            } else {
                println!("invalid option")
            }
            life += data.life;
            println!("life variation: {}", data.life);
            println!("");
        } else {
            break;
        }

        if life <= 0 {
            println!("You have die!");
            break;
        }
    }
}
