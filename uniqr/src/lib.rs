use clap::{Arg, Command};
use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust implementation of uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT FILE")
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .default_value("false")
                .num_args(0),
        )
        .get_matches();

    let in_file = matches
        .get_one::<String>("in_file")
        .map(String::from)
        .unwrap();

    let out_file = matches.get_one::<String>("out_file").map(String::from);

    let count = matches.get_flag("count");

    Ok(Config {
        in_file,
        out_file,
        count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
