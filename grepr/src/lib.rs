use clap::{Arg, Command};
use regex::{Regex, RegexBuilder};
use std::error::Error;

type MyResult<T> = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Recursive search")
                .default_value("false"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .default_value("false"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .default_value("false"),
        )
        .arg(
            Arg::new("invert-match")
                .short('v')
                .long("invert-match")
                .help("Invert match")
                .default_value("false"),
        )
        .get_matches();

    Ok(Config {
        pattern,
        files,
        recursive,
        count,
        invert_match,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
