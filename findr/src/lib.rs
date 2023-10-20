use crate::EntryType::*;
use regex::Regex;
use std::error::Error;

use clap::{App, Arg};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    File,
    Directory,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> GenericResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Richard Hoffmann <rhoffmann@fastmail.com>")
        .about("find in rust")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .default_value(".")
                .multiple(true)
                .help("Search paths"),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .takes_value(true)
                .multiple(true)
                .help("Name"), // .takes_value(false),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .takes_value(true)
                .multiple(true)
                .possible_values(&["f", "d", "l"])
                .help("Entry type"),
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|values| {
            values
                .into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|values| {
            values
                .iter()
                .map(|val| match val.as_str() {
                    "d" => Directory,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: vec![],
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> GenericResult<()> {
    println!("{:?}", config);
    Ok(())
}
