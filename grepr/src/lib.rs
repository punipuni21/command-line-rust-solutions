use clap::{Arg, Command};
use regex::{Regex, RegexBuilder};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}
