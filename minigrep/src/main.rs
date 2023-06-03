use std::{env, process};

use minigrep::Config;

fn main() {
    println!("====== Welcome to Mini-grep! ======");

    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
