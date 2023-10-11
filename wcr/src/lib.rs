use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{App, Arg};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> GenericResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Richard Hoffmann <rhoffmann@fastmail.com>")
        .about("wc in rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .multiple(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show char count")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: true,
        words: true,
        bytes: true,
        chars: false,
    })
}

pub fn run(config: Config) -> GenericResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn open(filename: &str) -> GenericResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
