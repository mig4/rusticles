use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("With text:\n{}", contents);

    Ok(())
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
}
