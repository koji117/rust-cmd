use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
pub struct Args {
    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,
    /// Number of lines
    #[arg(
    short('n'),
    long,
    default_value = "10",
    value_name = "LINES",
    value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,
    /// Number of bytes
    #[arg(
    short('c'),
    long,
    value_name = "BYTES",
    conflicts_with("lines"),
    value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

pub fn run() -> MyResult<()> {
    let args = crate::Args::parse();
    let args = Args {
        files: args.files,
        lines: args.lines,
        bytes: args.bytes,
    };
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file_content) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" },)
                }
                print_file_content(file_content, args.lines, args.bytes)?
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // The compiler doesn’t have enough information from dyn BufRead to know the size of the return type so wrap it with Box to store it in heap
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_file_content(
    mut file_content: Box<dyn BufRead>,
    lines: u64,
    bytes: Option<u64>,
) -> MyResult<()> {
    // println!("==> {} <==", file_name);
    // Use pattern matching to check if bytes is Some number of bytes to read.
    if let Some(num_bytes) = bytes {
        // When _ is used in a type annotation, it tells the compiler to infer the type.
        let bytes: Result<Vec<_>, _> = file_content.bytes().take(num_bytes as usize).collect();

        print!(
            "{}",
            // lossy will replace invalid UTF8 bytes to a special char
            String::from_utf8_lossy(&bytes?)
        )
    } else {
        // empty mutable string buffer to hold each line.
        let mut line = String::new();
        for _ in 0..lines {
            // read_line can modify the internal state of the file_content object, so file_content needs to be mutable
            // BufRead::read_line to read the next line into the string buffer.
            let bytes = file_content.read_line(&mut line)?;
            // file_content returns zero bytes when it reaches the end of the file, so break out of the loop.
            if bytes == 0 {
                break;
            }
            print!("{line}");
            // Use String::clear to empty the line buffer.
            line.clear();
        }
    }
    Ok(())
}
