fn main() {
    if let Err(e) = findr::get_args().and_then(findr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
