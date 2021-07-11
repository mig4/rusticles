use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_insensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let search_fn =
        if config.case_insensitive { search } else { search_case_insensitive };

    for line in search_fn(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|l| l.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_parses_args() {
        let config = Config::new(&[
            "bin".to_owned(), "pattern".to_owned(), "filename".to_owned()
        ]).unwrap();
        assert_eq!(config.query, "pattern");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    #[should_panic]
    fn config_new_fails_on_not_enough_args() {
        let _ = Config::new(&["bin".to_owned()]).unwrap();
    }

    #[test]
    fn search_is_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn search_returns_an_empty_result_when_no_matches() {
        let query = "unfindable";
        let contents = "\
All the lines
That can be found
here.";

        assert_eq!(Vec::new() as Vec<&str>, search(query, contents));
    }

    #[test]
    fn search_case_insensitive_is_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
