use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> GenericResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Richard Hoffmann <rhoffmann@fastmail.com>")
        .about("uniq in rust")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .default_value("-")
                .help("Input file"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Show count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        // in_file: matches.value_of_lossy("in_file").map(String::from).unwrap(),
        // in_file: matches.value_of_lossy("in_file").map(Into::into).unwrap(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> GenericResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn open(filename: &str) -> GenericResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
