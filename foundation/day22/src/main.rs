use std::{env, process};

use crate::app::run::run;
use crate::cli::args::{Command, ParseOutcome};
use crate::cli::{args::Args, error::CliError};

mod app;
mod cli;
mod domain;
mod persist;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = match std::env::args().next() {
        Some(name) => name,
        None => "invctl".to_string(),
    };

    match Args::parse(std::env::args()) {
        Ok(ParseOutcome::Help) => {
            println!("{}", crate::cli::help::help(&program));
            process::exit(0);
        }
        Ok(ParseOutcome::Args(args)) => {
            let out = crate::app::run::run(args)?;
            println!("{}", out);
            process::exit(0);
        }
        Err(e) => {
            let cli_err = CliError::Usage(e);
            eprintln!("{}", cli_err.render(&program));
            process::exit(cli_err.exit_code());
        }
    }
}
