use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        // ignore argument 0 as that i the path of the calling process
        let query = args[1].clone();
        let file_path: String = args.get(2).cloned().unwrap_or_else(|| ".".to_string());
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
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

pub fn search_directory(config: Config) -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();

    for entry in fs::read_dir(config.file_path)? {
        let entry = entry?;
        let path = entry.path();

        // non-recursive: anything that isn't a regular file gets skipped, never descended into
        if !path.is_file() {
            continue;
        }

        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let path_str = path.to_string_lossy().to_string();

        let mut matches = if config.ignore_case {
            search_case_insensitive(&config.query, &contents, &path_str)
        } else {
            search(&config.query, &contents, &path_str)
        };

        results.append(&mut matches);
    }

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search_file(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents, &config.file_path)
    } else {
        search(&config.query, &contents, &config.file_path)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
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
        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "frog");
        assert_eq!(config.file_path, "poems/poem.txt");
    }

    #[test]
    fn build_defaults_path_to_current_dir() {
        let args = vec!["minigrep".to_string(), "frog".to_string()];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.file_path, ".");
    }

    #[test]
    fn build_errors_without_query() {
        let args = vec!["minigrep".to_string()];
        assert!(Config::build(&args).is_err());
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
