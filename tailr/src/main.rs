fn main() {
    if let Err(e) = tailr::get_args().and_then(tailr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
