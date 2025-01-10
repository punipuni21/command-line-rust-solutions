use crate::EntryType::*;
use clap::{Arg, Command};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

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
    entry_types: Vec<EntryType>,
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
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
                .num_args(0..),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .help("Entry type")
                .short('t')
                .long("type")
                // possible_valuesからvalue_parserに変更されている
                .value_parser(clap::builder::PossibleValuesParser::new(["f", "d", "l"]))
                .num_args(0..),
        )
        .arg(
            Arg::new("paths")
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

    let entry_types = matches
        .get_many::<String>("types")
        .map(|vals| {
            vals.into_iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}
