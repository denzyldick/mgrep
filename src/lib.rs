use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query specified"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename specified"),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "helloworld";
        let contents = "\
  Lorem Ipsum is simply dummy
  text of the printing and typesetting industry.
  Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown
  printer took a galley of type and scrambled it to
  make a type helloworld
  specimen book.
  It has survived not only five centuries, but also the leap into electronic
  typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of
  Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing
   software like Aldus PageMaker including versions of Lorem Ipsum.
    ";

        assert_eq!(vec![
            "  make a type helloworld"
        ],
                   search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "a";

        let contents = "\
       a
       b
       c
        ";

        assert_eq!(vec!["a"], search(query, contents))
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase()))
        .collect()
}
