use minigrep::{Config, search_directory, search_file};
use std::env;
use std::path::Path;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for the word: '{}, in file {}\n",
        config.query, config.file_path
    );

    let file_path = Path::new(&config.file_path);

    if file_path.is_dir() {
        if let Err(e) = search_directory(config) {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    } else if file_path.is_file() {
        if let Err(e) = search_file(config) {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    } else {
        panic!("Filepath does not exist!")
    }
}
