use crate::EntryType::*;
use clap::{Arg, Command};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_type: EntryType,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .about("Find files and directories")
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .help("Name")
                .short('n')
                .long("name")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .help("Entry type")
                .short('t')
                .long("Type")
                .value_parser(clap::builder::PossibleValuesParser::new(["f", "d", "l"]))
                .num_args(1..),
        )
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(1..),
        )
        .get_matches();

    let names = matches
        .get_many::<String>("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let paths: Vec<String> = matches
        .get_many::<String>("paths")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let entry_type = matches
        .get_one::<String>("types")
        .map(|vals| match vals.as_str() {
            "f" => Dir,
            "d" => File,
            "l" => Link,
            _ => unreachable!("Invalid type"),
        })
        .unwrap();

    Ok(Config {
        paths,
        names,
        entry_type,
    })
}
