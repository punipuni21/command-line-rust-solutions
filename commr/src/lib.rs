use clap::{Arg, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

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
                .required(true),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .help("Suppress printing of column 1")
                .default_value("true")
                .num_args(0),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .help("Suppress printing of column 2")
                .default_value("true")
                .num_args(0),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .help("Suppress printing of column 3")
                .default_value("true")
                .num_args(0),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .help("Case-insensitive comparisn of lines")
                .default_value("false")
                .num_args(0),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t"),
        )
        .get_matches();

    let file1 = matches.get_one::<String>("file1").unwrap().to_string();
    let file2 = matches.get_one::<String>("file2").unwrap().to_string();
    let delimiter = matches.get_one::<String>("delimiter").unwrap().to_string();

    Ok(Config {
        file1,
        file2,
        show_col1: !matches.get_flag("suppress_col1"),
        show_col2: !matches.get_flag("suppress_col2"),
        show_col3: !matches.get_flag("suppress_col3"),
        insensitive: matches.get_flag("insensitive"),
        delimiter,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().find_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().find_map(Result::ok).map(case);

    let mut line1 = lines1.iter().next();
    let mut line2 = lines2.iter().next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(_), Some(_)) => {
                line1 = lines1.iter().next();
                line2 = lines2.iter().next();
            }
            (Some(_), None) => {
                line1 = lines1.iter().next();
            }
            (None, Some(_)) => {
                line2 = lines2.iter().next();
            }
            _ => (),
        };
    }
    Ok(())
}
