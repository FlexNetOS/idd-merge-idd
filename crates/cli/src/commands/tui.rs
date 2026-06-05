//! `rusty-idd tui` — launch the interactive OpenSpec TUI via the shared
//! `openspec_tui` library (the same implementation the standalone
//! `openspec-tui` binary uses).

/// Launch the interactive TUI. Returns a process exit code.
pub fn run() -> i32 {
    match openspec_tui::run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("rusty-idd: tui error: {err}");
            1
        }
    }
}
