use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_number, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_number + 1, line);
                        continue;
                    }

                    if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                            continue;
                        }

                        println!("{:>6}\t{}", last_num + 1, line);
                        last_num += 1;
                        continue;
                    }

                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("author")
        .about("Rust cat")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("number all output lines")
                .conflicts_with("number_nonblank_lines")
                .num_args(0),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines")
                .num_args(0),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many::<String>("file")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    Ok(Config {
        files: text,
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
