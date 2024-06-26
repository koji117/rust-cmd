use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

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
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
            EntryType::Link => entry.file_type().is_symlink(),
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
        })
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
            .names
            .iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &args.paths {
        // Turn WalkDir into an iterator and use Iterator::filter_map to remove and print bad results to STDERR
        // while allowing Ok results to pass through.
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
