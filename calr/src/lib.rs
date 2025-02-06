use chrono::NaiveDate;
use clap::{Arg, Command};
use std::error::Error;

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

    Ok(Config {
        month: None,
        year: 12,
        today: NaiveDate::from_ymd(2021, 1, 1),
    })
}
