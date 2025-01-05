use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .about("head")
        .author("ryo")
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .value_name("LINES")
                .help("Show line count")
                .default_value("true")
                .num_args(0),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .value_name("WORDS")
                .help("Show word count")
                .default_value("true")
                .num_args(0),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .default_value("true")
                .num_args(0),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .value_name("CHARS")
                .help("Show character count")
                .conflicts_with("bytes")
                .default_value("false")
                .num_args(0),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .get_matches();

    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}
