use minigrep::{Config, search_directory, search_file};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for the word: '{}, in file {}\n",
        config.query, config.file_path
    );

    let file_path = Path::new(&config.file_path);

    if file_path.is_dir() {
        match search_directory(
            &config.query,
            &config.file_path,
            config.ignore_case,
            config.show_hidden,
        ) {
            Ok(results) => {
                for line in results {
                    println!("{line}");
                }
            }
            Err(e) => {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
    } else if file_path.is_file() {
        match search_file(config) {
            Ok(results) => {
                for line in results {
                    println!("{line}");
                }
            }
            Err(e) => {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
    } else {
        panic!("Filepath does not exist!")
    }
}
