use anyhow::Ok;
use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .about("head")
        .author("ryo")
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Input lines")
                .default_value("10")
                .num_args(0),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Input bytes")
                .num_args(0),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many::<String>("lines")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    Ok(Config {
        files: text,
        lines: 10,
        bytes: None,
    })
}
