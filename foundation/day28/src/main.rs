use day28::app::output::AppOutput;
use day28::config::Config;
use day28::{app, cli};

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

    let config = Config::load(file);
    let out = match app::run(config.storage_path, cmd) {
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
