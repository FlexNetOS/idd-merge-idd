fn main() {
    if let Err(err) = intent_driven_development::run_from_env() {
        eprintln!("idd: {err}");
        std::process::exit(1);
    }
}
