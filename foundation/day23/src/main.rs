use std::path::Path;
use std::process;

use day23::app::run::run;
use day23::app::settings::resolve_effective_settings;
use day23::cli::args::{Args, ParseOutcome};
use day23::cli::error::{CliError, RunError};
use day23::config::Config;
use day23::env::{Env, OsEnv};

struct CliEnv<'a, E> {
    base: &'a E,
    args: &'a Args,
}

impl<E: Env> Env for CliEnv<'_, E> {
    fn get(&self, key: &'static str) -> Option<String> {
        let override_val = match key {
            "DATA_FILE" => self
                .args
                .file
                .as_ref()
                .map(|path| path.to_string_lossy().to_string()),
            "PAGE_SIZE" => self.args.page_size.map(|v| v.to_string()),
            _ => None,
        };

        override_val.or_else(|| self.base.get(key))
    }
}

fn main() {
    let _ = dotenvy::dotenv();
    let program = match std::env::args().next() {
        Some(name) => name,
        None => "invctl".to_string(),
    };

    match Args::parse(std::env::args()) {
        Ok(ParseOutcome::Help) => {
            println!("{}", day23::cli::help::help(&program));
            process::exit(0);
        }
        Ok(ParseOutcome::Args(args)) => {
            let env = OsEnv;
            let config_path = Path::new("config.toml");
            let cli_env = CliEnv { base: &env, args: &args };
            let cfg = match Config::load(&cli_env, config_path) {
                Ok(cfg) => cfg,
                Err(err) => {
                    let cli_err = CliError::Run(RunError::from(err));
                    eprintln!("{}", cli_err.render(&program));
                    process::exit(cli_err.exit_code());
                }
            };
            let effective = resolve_effective_settings(&args, &cfg);
            let mut args = args;
            args.file = Some(effective.data_file);
            args.page_size = Some(effective.page_size);

            match run(args) {
                Ok(out) => {
                    println!("{}", out);
                    process::exit(0);
                }
                Err(e) => {
                    let cli_err = CliError::Run(e);
                    eprintln!("{}", cli_err.render(&program));
                    process::exit(cli_err.exit_code());
                }
            }
        }
        Err(e) => {
            let cli_err = CliError::Usage(e);
            eprintln!("{}", cli_err.render(&program));
            process::exit(cli_err.exit_code());
        }
    }
}
