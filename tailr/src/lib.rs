use crate::TakeValue::*;
use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust tail")
        .get_matches();

    Ok(Config {
        files: Vec::new(),
        lines: PlusZero,
        bytes: None,
        quiet: false,
    })
}
