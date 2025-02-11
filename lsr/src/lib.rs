use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("lsr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust ls")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .num_args(1..)
                .default_value("."),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Long listing")
                .default_value("false")
                .num_args(0),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Show all files")
                .default_value("false")
                .num_args(0),
        )
        .get_matches();

    let paths = matches
        .get_many::<String>("paths")
        .unwrap()
        .map(String::from)
        .collect();

    Ok(Config {
        paths: paths,
        long: matches.get_flag("long"),
        show_hidden: matches.get_flag("all"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
