use crate::Extract::*;
use clap::{builder::Str, Arg, Command};
use std::{error::Error, ops::Range};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("curt")
        .version("0.1.0")
        .author("ryo")
        .about("Rust cut")
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('b')
                .long("bytes")
                .help("Selected bytes")
                .num_args(0..),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('c')
                .long("chars")
                .help("Selected characters")
                .num_args(0..),
        )
        .arg(
            Arg::new("delim")
                .value_name("DELIMITER")
                .short('d')
                .long("delim")
                .help("Field delimiter")
                .default_value(" ")
                .num_args(0..),
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .short('f')
                .long("fields")
                .help("Selected fields")
                .num_args(0..),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let delimiter = matches
        .get_one::<String>("lines")
        .map(|s| s.as_str()) //String->&str
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| {
            // 本とエラーメッセージが違うので注意
            format!(
                "error: invalid value '{e}' for \
          '--lines <LINES>': invalid digit found in string"
            )
        })?;
}
