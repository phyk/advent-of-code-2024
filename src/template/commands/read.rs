use std::process;

use crate::template::{aoc_cli, Day};

pub fn handle(day: Day, session_file: &str) {
    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    if let Err(e) = aoc_cli::read(day, session_file) {
        eprintln!("failed to call aoc-cli: {e}");
        process::exit(1);
    };
}
