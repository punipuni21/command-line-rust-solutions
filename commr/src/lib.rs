use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("commr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust comm")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true)
                .default_value("true"),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true)
                .default_value("true"),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .help("Suppress printing of column 1")
                .default_value("true")
                .required(false),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .help("Suppress printing of column 2")
                .default_value("true")
                .required(false),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .help("Suppress printing of column 3")
                .default_value("true")
                .required(false),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .help("Case-insensitive comparisn of lines")
                .default_value("false")
                .required(false),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t")
                .required(true),
        )
        .get_matches();

    Ok(Config {
        file1: "file1".to_string(),
        file2: "file2".to_string(),
        show_col1: true,
        show_col2: true,
        show_col3: true,
        insensitive: false,
        delimiter: "\t".to_string(),
    })
}
