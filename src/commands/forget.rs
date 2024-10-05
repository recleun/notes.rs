use std::{fs, process};

pub struct Forget;
pub struct Arguments {
    identifier: String,
}

impl super::Command for Forget {
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
        let mut json: serde_json::Map<String, serde_json::Value> = serde_json::from_reader(reader).expect("JSON file is poorly formatted");
        if json.contains_key(&args.identifier) {
            json.remove(&args.identifier);
        } else {
            eprintln!("Specified key was not found: {}", args.identifier);
            process::exit(-1);
        }
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
                println!("Note has been forgotten successfully");
                process::exit(0);
            },
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
