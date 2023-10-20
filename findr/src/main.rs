fn main() {
    if let Err(e) = findr::get_args().and_then(findr::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1)
    }
}
