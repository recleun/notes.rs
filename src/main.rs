mod commands;

use commands::Command;
use std::{env, process};

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        eprintln!("No arguments given");
        process::exit(-1)
    }

    let command = args.remove(0);
    match command.as_str() {
        "get" => {
            commands::Get::run(args);
        },
        "set" => {
            commands::Set::run(args);
        },
        "forget" => {
            commands::Forget::run(args);
        },
        "list" => {
            commands::List::run(Vec::new());
        },
        _ => eprintln!("Unknown command: {}", command),
    }
}
