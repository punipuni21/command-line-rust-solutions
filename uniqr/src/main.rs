fn main() {
    if let Err(e) = uniqr::get_args().and_then(uniqr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
