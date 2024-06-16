use clap::builder::PossibleValue;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;

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
    #[arg(value_name = "PATH", default_value = ".", last = true)]
    paths: Vec<String>,

    /// Name
    #[arg(
        value_name = "NAME",
        short('n'), long("name"),
        value_parser(Regex::new), // Uses Regex::new to parse input values into Regex objects
        action(ArgAction::Append), // Appends each occurrence to the names vector
        num_args(0..)
    )]
    names: Vec<Regex>,

    #[arg(
        value_name = "TYPE",
        short('t'), long("type"),
        value_parser = clap::value_parser!(EntryType), // Parse input values to EntryType (File, Dir or Link)
        num_args(0..) // // Appends each occurrence to the types vector
    )]
    entry_types: Vec<EntryType>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
