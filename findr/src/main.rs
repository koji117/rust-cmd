use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `find`
struct Args {
    // Search path(s)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// File names
    #[arg(
        value_name = "NAME",
        short('n'), long("name"),
        value_parser(Regex::new), // Uses Regex::new to parse input values into Regex objects
        action(ArgAction::Append), // Appends each occurrence to the names vector
        num_args(0..)
    )]
    names: Vec<Regex>,

    // Entry types, FILE and(or) LINK
    #[arg(
        value_name = "TYPE",
        short('t'), long("type"),
        value_parser = clap::value_parser!(EntryType), // Parse input values to EntryType (File, Dir or Link)
        action(ArgAction::Append),
        num_args(0..) // // Appends each occurrence to the types vector
    )]
    entry_types: Vec<EntryType>,
}

fn main() {
    let execution_result = run(Args::parse());

    match execution_result {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(()) => {
            // println!("BIG SUCCESS")
        }
    }
}

fn run(args: Args) -> Result<()> {
    for path in &args.paths {
        // println!("{:?}", args);
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("Error: {e}"),
                Ok(entry) => {
                    if args.entry_types.is_empty() {
                        println!("{}", entry.path().display());
                    }
                    if entry.path().is_dir() && args.entry_types.contains(&EntryType::Dir) {
                        let path_str = entry.path().to_str().unwrap().to_string();
                        println!("{}", path_str);
                    }
                    if entry.path().is_file() && args.entry_types.contains(&EntryType::File) {
                        let path_str = entry.path().to_str().unwrap().to_string();
                        println!("{}", path_str);
                    }
                    if entry.path().is_symlink() && args.entry_types.contains(&EntryType::Link) {
                        let path_str = entry.path().to_str().unwrap().to_string();
                        println!("{}", path_str);
                    }
                }
            }
        }
    }
    Ok(())
}
