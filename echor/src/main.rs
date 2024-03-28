use clap::{App, Arg};

fn main() {
    // println!("{:?}", std::env::args()); // Success at last!
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Koji Saruya")
        .about("Rust Echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    // unwrap() can be safely used as "text" is required in Arg, so it can never be None
    // If you call Option::unwrap on a None, it will cause a panic that will crash your program.
    // You should only call unwrap if you are positive the value is Some variant.
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
