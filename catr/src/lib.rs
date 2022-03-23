use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Wonsang Chong <temporaryaion1@hotamil.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("filenames")
                .value_name("FILENAMES")
                .help("Names of input files")
                .required(true)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .value_name("NUMBER_LINES")
                .short("n")
                .long("number")
                .help("whether lines are numbered or not")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .value_name("number non-blank lines")
                .short("b")
                .long("number-nonblank")
                .conflicts_with("number_lines")
                .help("whether blank lines are numbered or not")
                .takes_value(false),
        )
        .get_matches();
    let filenames = matches.values_of_lossy("filenames").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    Ok(Config {
        files: filenames,
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank_lines,
    })
}
