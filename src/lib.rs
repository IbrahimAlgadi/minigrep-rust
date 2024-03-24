use std::env;
use std::fs;
use std::process;
use std::error::Error;


#[derive(Debug)]
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
            None => return Err("Didn't get query parameter")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the file path")
        };
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}


pub fn run(config: Config)  -> Result<(), Box<dyn Error>> {
    let contents =
    fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    if config.ignore_case {
        for line in search_insensitive(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    }
    
    Ok(())
}

// fn parse_config(args: &[String]) -> Config {
//     Config {
//         query: args[0].clone(),
//         file_path: args[2].clone(),
//     }
// }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // let mut results = vec![];
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
}


pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
    // let mut results = vec![];
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_insensitive(query, contents)
        )
    }
}