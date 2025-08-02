use colored::{self, Colorize};
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        let formatted_line = highlight_matches(line, &config.query, &config.ignore_case);
        println!("{}", formatted_line);
    }

    Ok(())
}

fn highlight_matches(line: &str, query: &str, ignore_case: &bool) -> String {
    if *ignore_case {
        let line_lower = line.to_lowercase();
        let query_lower = query.to_lowercase();
        let mut result = String::new();
        let mut end = 0;

        for (idx, string) in line_lower.match_indices(&query_lower) {
            result.push_str(&line[end..idx]);
            result.push_str(&string.red().to_string());
            end = idx + query.len();
        }
        result.push_str(&line[end..]);
        result
    } else {
        line.replace(query, &query.red().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "tEsT";
        let contents = "\
Hello there
fellow rustaceans
this is a simple minigrep TEST";

        assert_eq!(
            vec!["this is a simple minigrep TEST"],
            search_case_insensitive(query, contents)
        );
    }
}
