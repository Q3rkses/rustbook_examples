use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;

const MAX_FILE_SIZE: u64 = 1 * 1024 * 1024;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_hidden: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => ".".to_string(),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let show_hidden = env::var("SHOW_HIDDEN").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            show_hidden,
        })
    }
}
pub fn search<'a>(query: &str, contents: &'a str, file_path: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        if line.contains(&query) {
            results.push(
                line.replace(&query, &query.bold().red().to_string())
                    + " located in file, "
                    + &file_path.to_string()
                    + " at line "
                    + &idx.to_string(),
            );
        }
    }
    results
}
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    file_path: &'a str,
) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        let line = line.to_lowercase();
        if line.contains(&query) {
            results.push(
                line.replace(&query, &query.bold().red().to_string())
                    + " in file: "
                    + &file_path.to_string()
                    + ", line: "
                    + &idx.to_string(),
            );
        }
    }
    results
}

pub fn search_directory(
    query: &str,
    dir: &str,
    ignore_case: bool,
    show_hidden: bool,
) -> std::io::Result<Vec<String>> {
    let mut results = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_symlink() {
            continue;
        }

        if !show_hidden {
            let name = entry.file_name();
            if name.to_string_lossy().starts_with('.') {
                continue;
            }
        }

        if path.is_dir() {
            let sub_dir = path.to_string_lossy();
            let mut sub_results = search_directory(query, &sub_dir, ignore_case, show_hidden)?;
            results.append(&mut sub_results);
        } else if path.is_file() {
            if entry.metadata()?.len() > MAX_FILE_SIZE {
                continue;
            }

            let contents = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let path_str = path.to_string_lossy().to_string();

            let mut matches = if ignore_case {
                search_case_insensitive(query, &contents, &path_str)
            } else {
                search(query, &contents, &path_str)
            };

            results.append(&mut matches);
        }
    }
    Ok(results)
}

pub fn search_file(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents, &config.file_path)
    } else {
        search(&config.query, &contents, &config.file_path)
    };
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    #[test]
    fn build_reads_query_and_path() {
        let args = vec![
            "minigrep".to_string(),
            "frog".to_string(),
            "poems/poem.txt".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();

        assert_eq!(config.query, "frog");
        assert_eq!(config.file_path, "poems/poem.txt");
    }

    #[test]
    fn build_defaults_path_to_current_dir() {
        let args = vec!["minigrep".to_string(), "frog".to_string()];
        let config = Config::build(args.into_iter()).unwrap();

        assert_eq!(config.file_path, ".");
    }

    #[test]
    fn build_errors_without_query() {
        let args = vec!["minigrep".to_string()];
        assert!(Config::build(args.into_iter()).is_err());
    }

    #[test]
    fn case_sensitive_finds_one_line() {
        let results = search("Rust", CONTENTS, "poem.txt");

        assert_eq!(results.len(), 1);
        // idx is 0-based, so the first line reports "line 0"
        assert!(results[0].contains("at line 0"));
        assert!(results[0].contains("poem.txt"));
    }

    #[test]
    fn case_sensitive_skips_wrong_case() {
        // "RUST" (all caps) should not match "Rust" or "Trust"
        let results = search("RUST", CONTENTS, "poem.txt");
        assert!(results.is_empty());
    }

    #[test]
    fn case_insensitive_finds_both_cases() {
        // "rUsT" should match both "Rust" and "Trust"
        let results = search_case_insensitive("rUsT", CONTENTS, "poem.txt");
        assert_eq!(results.len(), 2);
    }
}
