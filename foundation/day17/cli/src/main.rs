use catalog::store::catalog::Catalog;
use cli::app::engine::execute;
use cli::app::view::Response;
use cli::cli::command::Command;
use cli::cli::render::{render, render_error};
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let mut catalog = Catalog::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let cmd = match Command::from_str(trimmed) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("{}", render_error(e));
                continue;
            }
        };

        if matches!(cmd, Command::Exit) {
            match execute(&mut catalog, cmd) {
                Ok(resp) => {
                    println!("{}", render(resp));
                    break;
                }
                Err(e) => {
                    println!("{}", render_error(e));
                    break;
                }
            }
        }

        match execute(&mut catalog, cmd) {
            Ok(resp) => {
                if matches!(resp, Response::Exit) {
                    println!("{}", render(resp));
                    break;
                } else {
                    println!("{}", render(resp));
                }
            }
            Err(e) => {
                println!("{}", render_error(e));
            }
        }
    }
}
