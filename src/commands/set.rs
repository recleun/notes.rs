use std::{process, fs};

pub struct Set;
pub struct Arguments {
    identifier: String,
    value: String,
}

impl super::Command for Set {
    type Args = Arguments;
    fn run_with_args(args: Self::Args) {
        if !fs::exists("notes.json").unwrap() {
            match fs::write("notes.json", "{}") {
                Err(e) => {
                    eprintln!("Failed to create notes.json file: {}", e);
                    process::exit(-1);
                }
                _ => (),
            }
        }
        let reader = match fs::File::open("notes.json") {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("Failed to open notes.json file: {}", e);
                process::exit(-1);
            }
        };
        let mut json: serde_json::Map<String, serde_json::Value> = serde_json::from_reader(reader).expect("JSON file is poorly formatted");
        if json.contains_key(&args.identifier) {
            json.remove(&args.identifier);
        }
        json.insert(args.identifier, serde_json::to_value(args.value).unwrap());
        let writer = match fs::File::create("notes.json") {
            Ok(reader) => reader,
            Err(e) => {
                eprintln!("Failed to open notes.json file for write: {}", e);
                process::exit(-1);
            }
        };
        match serde_json::to_writer(writer, &json) {
            Err(e) => {
                eprintln!("Failed to write to notes.json file: {}", e);
                process::exit(-1);
            }
            _ => {
                println!("Note has been written successfully");
                process::exit(0);
            },
        }
    }
}

impl super::Arguments for Arguments {
    fn parse(args: Vec<String>) -> Self {
        if args.is_empty() {
            eprintln!("No command options given");
            process::exit(-1);
        }
        if args.len() < 2 {
            eprintln!("Not enough command options were specified");
            process::exit(-1);
        }
        Arguments {
            identifier: args[0].clone(),
            value: args.into_iter()
                .skip(1)
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}
