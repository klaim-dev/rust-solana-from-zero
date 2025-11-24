use day08::refactored::config::parse_config;

fn main() {
    let config_data = "\
host=localhost
port=8080
use_tls=true
timeout_ms=5000
";

    match parse_config(config_data) {
        Ok(cfg) => println!("Parsed config: {:?}", cfg),
        Err(e) => eprintln!("Error parsing config: {}", e),
    }
}
