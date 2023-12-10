use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Lines containing query `{}` in file `{}` (case {}):", config.query, config.filename, if config.case_sensitive {"sensetive"} else {"insensetive"});

    if let Err(e) = print_found_lines(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}

fn print_found_lines(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let lines = search(&config.query, &contents, config.case_sensitive);

    for line in lines {
        println!("- {}", line);
    }

    Ok(())
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
    case_sensitive: bool,
}

impl Config<'_> {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[2];
        let filename = &args[1];

        let mut case_sensitive = true;
        if args.len() == 4 && &args[3] == "i" {
            case_sensitive = false;
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let query: Cow<str> = if case_sensitive {
        Cow::Borrowed(query)
    } else {
        Cow::Owned(query.to_lowercase())
    };

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if !case_sensitive && line.to_lowercase().contains(query.as_ref())
            || case_sensitive && line.contains(query.as_ref())
        {
            results.push(line);
        }
    }

    results
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
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
