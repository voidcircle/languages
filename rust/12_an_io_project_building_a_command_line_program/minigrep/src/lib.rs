use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub term: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(user_args: &[String]) -> Result<Config, &'static str> {
        if user_args.len() != 3 {
            return Err("The arguments are not valid.");
        }

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            term: user_args[1].to_string(),
            file: user_args[2].to_string(),
            ignore_case,
        })
    }
}

pub fn run(configuration: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(configuration.file)?;

    let searched_lines: Vec<&str>;

    if configuration.ignore_case {
        searched_lines = search_case_insensitive(&configuration.term, &contents);
    } else {
        searched_lines = search(&configuration.term, &contents);
    }

    if searched_lines.len() == 0 {
        println!("*FOUND NOTHING*");
    } else {
        for (current_line_index, current_line) in searched_lines.into_iter().enumerate() {
            println!("{}: {current_line}", current_line_index + 1);
        }
    }

    Ok(())
}

pub fn search<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for current_line in contents.lines() {
        if current_line.contains(term) {
            results.push(current_line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for current_line in contents.lines() {
        if current_line.to_lowercase().contains(&term.to_lowercase()) {
            results.push(current_line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_query() {
        let term = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(term, contents));
    }

    #[test]
    fn name() {
        let term = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(term, contents)
        );
    }
}
