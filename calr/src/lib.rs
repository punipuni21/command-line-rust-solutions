use chrono::{Datelike, Local, NaiveDate};
use clap::{Arg, Command};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("calr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust cal")
        .get_matches();

    let today = Local::today();

    Ok(Config {
        month: None,
        year: today.year(),
        today: today.naive_local(),
    })
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    unimplemented!();
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
