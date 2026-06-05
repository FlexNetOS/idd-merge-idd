//! Thin binary shim for the standalone `openspec-tui` executable. The actual
//! implementation lives in the `openspec_tui` library crate (shared with the
//! unified `rusty-idd tui` subcommand).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    openspec_tui::run()
}
