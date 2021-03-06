use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.senstitive {
        search(&config.query, &contents)
    } else {
        search_insenstitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub senstitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't fet a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't fet a file name"),
        };
        //使用环境变量
        let senstitive = env::var("SENSTITIVE").is_err();
        Ok(Config {
            query,
            filename,
            senstitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     };
    // }
    // result

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insenstitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         result.push(line);
    //     };
    // };
    // result

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_senstitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insenstitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insenstitive(query, contents)
        );
    }
}
