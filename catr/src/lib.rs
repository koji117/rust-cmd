use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Koji Saruya")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE(S)")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Print line number")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Print nonblank line number")
                .takes_value(false),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file_content) => print_file_content(
                file_content,
                config.number_lines,
                config.number_nonblank_lines,
            )?,
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_file_content(
    file_content: Box<dyn BufRead>,
    number_lines: bool,
    number_nonblank_lines: bool,
) -> MyResult<()> {
    // Initialize a mutable variable for the number of the last nonblank line
    let mut last_num = 0;
    for (line_num, line) in file_content.lines().enumerate() {
        // Reusing line in this context is more Rustic code you’re likely to encounter.
        let line = line?;

        if number_lines {
            println!("{:>6}\t{}", line_num + 1, line);
        } else if number_nonblank_lines {
            if line.is_empty() {
                println!()
            } else {
                last_num += 1;
                println!("{:>6}\t{}", last_num, line);
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
