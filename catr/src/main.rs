use std::process;

fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
