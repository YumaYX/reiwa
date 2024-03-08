fn main() {
    if let Err(e) = reiwa::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
