fn main() {
    if let Err(e) = calr::get_args().and_then(calr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
