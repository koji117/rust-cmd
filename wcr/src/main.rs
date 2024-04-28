use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
struct Args {
    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,
    /// Whether to print the line count.
    #[arg(short('l'), long)]
    lines: bool,
    /// Whether to print the word count.
    #[arg(short('w'), long)]
    words: bool,
    /// Whether to print the byte count.
    #[arg(short('c'), long)]
    bytes: bool,
    /// Whether to print the char count.
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> MyResult<()> {
    // If all the flags are false, then set lines, words, and bytes to true.
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file_content) => {
                let info = count(file_content)?;
                println!(
                    "{}{}{}{}{}",
                    format_field(info.num_lines, args.lines),
                    format_field(info.num_words, args.words),
                    format_field(info.num_bytes, args.bytes),
                    format_field(info.num_chars, args.chars),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {filename}")
                    }
                );

                total_lines += info.num_lines;
                total_words += info.num_words;
                total_bytes += info.num_bytes;
                total_chars += info.num_chars;
            }
        }
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, args.lines),
            format_field(total_words, args.words),
            format_field(total_bytes, args.bytes),
            format_field(total_chars, args.chars)
        )
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn count(mut file_content: impl BufRead) -> MyResult<(FileInfo)> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file_content.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// The value that will be returned from the function is dynamically generated at runtime, so use String, which is a growable, heap-allocated structure.
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
