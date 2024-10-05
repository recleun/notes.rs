use std::{fs, process};

pub struct List;
pub struct Arguments;

impl super::Command for List {
    type Args = Arguments;

    fn run_with_args(_args: Self::Args) {
        if !fs::exists("notes.json").unwrap() {
            match fs::write("notes.json", "{}") {
                Err(e) => {
                    eprintln!("Failed to create notes.json file: {}", e);
                    process::exit(-1);
                }
                _ => {
                    println!("Notes are empty");
                    process::exit(0);
                },
            }
        }
        let reader = match fs::File::open("notes.json") {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("Failed to open notes.json file: {}", e);
                process::exit(-1);
            }
        };
        let json: serde_json::Map<String, serde_json::Value> = serde_json::from_reader(reader).expect("JSON file is poorly formatted");
        json.iter().for_each(| n | {
            let (key, value) = n;
            println!("{}: {}", key, value);
        });
    }
}

impl super::Arguments for Arguments {
    fn parse(_args: Vec<String>) -> Self {
        Arguments
    }
}
