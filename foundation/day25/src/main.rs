use std::path::PathBuf;

use day25::app::output::AppOutput;
use day25::app::service::OrderService;
use day25::cli;
use day25::persist::repo::FileRepo;

fn main() {
    let cli_cmd = match cli::parse_args(std::env::args().skip(1)) {
        Ok(cmd) => cmd,
        Err(err) => {
            if matches!(err, cli::UsageError::HelpRequested) {
                print!("{}", cli::usage());
                std::process::exit(0);
            }
            exit_usage(err);
        }
    };

    let (cmd, file) = match cli_cmd.into_app_command() {
        Ok(result) => result,
        Err(err) => exit_usage(err),
    };

    let path = file.unwrap_or_else(default_path);
    let repo = FileRepo::new(path);
    let service = OrderService::new(repo);

    let out = match service.run(cmd) {
        Ok(out) => out,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    match out {
        AppOutput::Text(text) => println!("{text}"),
    }
}

fn exit_usage(err: impl std::fmt::Display) -> ! {
    eprintln!("error: {err}");
    eprintln!("{}", cli::usage());
    std::process::exit(2);
}

fn default_path() -> PathBuf {
    if let Ok(value) = std::env::var("ORDERS_FILE") {
        if !value.trim().is_empty() {
            return PathBuf::from(value);
        }
    }
    PathBuf::from("orders.txt")
}
