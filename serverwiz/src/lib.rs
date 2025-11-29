//! Includes business logic for the application.

use clap::Parser;

mod cli;

/// Run the application.
pub fn run() {
    let arguments = cli::Cli::parse();

    let canon_full_path = arguments.canon_full_path().unwrap_or_else(|err| {
        println!("There was an error getting the path: {}", err);
        std::process::exit(1);
    });

    println!("Full path is: {}", canon_full_path.display());
}
