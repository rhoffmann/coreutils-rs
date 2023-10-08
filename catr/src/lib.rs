use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type ResultType<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> ResultType<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Richard Hoffmann <rhoffmann@fastmail.com>")
        .about("cat in rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .multiple(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> ResultType<()> {
    let mut num_line = 1;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Could not open file {}: {}", filename, err),
            Ok(reader) => {
                for line in reader.lines() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", num_line, line);
                        num_line += 1;
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            println!("{:>6}\t{}", num_line, line);
                            num_line += 1;
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> ResultType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
