use std::{fs, process};
use serde_json;

pub struct Get;
pub struct Arguments {
    identifier: String,
}

impl super::Command for Get {
    type Args = Arguments;

    fn run_with_args(args: Self::Args) {
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
        let json: serde_json::Value = serde_json::from_reader(reader).expect("JSON file is poorly formatted");
        if let Some(value) = json.get(&args.identifier) {
            println!("Note: {}", value.as_str().unwrap());
        } else {
            eprintln!("Identifier specified not found: {}", args.identifier);
            process::exit(-1);
        }
    }
}

impl super::Arguments for Arguments {
    fn parse(args: Vec<String>) -> Self {
        if args.is_empty() {
            eprintln!("No command options given");
            process::exit(-1)
        }
        Arguments {
            identifier: args[0].clone(),
        }
    }
}
