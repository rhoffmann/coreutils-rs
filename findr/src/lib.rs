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
            Arg::with_name("name")
                .value_name("NAME")
                .short("n")
                .long("name")
                .multiple(true)
                .help("Name"), // .takes_value(false),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .multiple(true)
                .possible_values(&["f", "d", "l"])
                .help("Entry type"),
        )
        .get_matches();

    Ok(Config {
        // paths: matches.value_of_lossy("paths").unwrap().to_string(),
        paths: vec![],
        names: vec![],
        entry_types: vec![],
    })
}

pub fn run(config: Config) -> GenericResult<()> {
    println!("{:?}", config);
    Ok(())
}
