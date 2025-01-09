use std::{
    env::{self, Args},
    error::Error,
    fs,
};

#[derive(Debug)]
pub struct Config {
    pub term: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut user_args: Args) -> Result<Config, &'static str> {
        user_args.next();

        Ok(Config {
            term: match user_args.next() {
                Some(user_term) => user_term,
                _ => return Err("The arguments are not valid."),
            },
            file: match user_args.next() {
                Some(user_file) => user_file,
                _ => return Err("The arguments are not valid."),
            },
            ignore_case: env::var("IGNORE_CASE").is_ok(),
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
    contents
        .lines()
        .filter(|current_line| current_line.contains(term))
        .collect()
}

pub fn search_case_insensitive<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {
    let term: String = term.to_lowercase();

    contents
        .lines()
        .filter(|current_line| current_line.to_lowercase().contains(&term))
        .collect()
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
