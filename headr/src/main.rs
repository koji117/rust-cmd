fn main() {
    if let Err(e) = headr::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
