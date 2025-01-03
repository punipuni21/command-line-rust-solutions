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
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .conflicts_with("lines")
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

    let files: Vec<String> = matches
        .get_many::<String>("lines")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let lines = matches
        .get_one("bytes")
        .map(parse_positive_int) //TODO implement parse_positive_int
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .get_one("bytes")
        .map(parse_positive_int) //TODO implement parse_positive_int
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: files,
        lines: lines.unwrap(),
        bytes: bytes,
    })
}
