use std::collections::HashMap;

use day15::build_config;
use day15::errors::ConfigError;

fn main() {
    println!("=== build_config manual checks ===");

    case(
        "happy path (all fields)",
        hm(&[
            ("DB_URL", "postgres://localhost"),
            ("PORT", "9000"),
            ("DEBUG", "true"),
            ("MAX_CONNECTIONS", "20"),
            ("MODE", "dev"),
        ]),
    );

    case(
        "defaults only (DB_URL only)",
        hm(&[("DB_URL", "postgres://localhost")]),
    );

    case("missing DB_URL", hm(&[("PORT", "8080")]));

    case("empty DB_URL", hm(&[("DB_URL", "")]));

    case(
        "invalid PORT",
        hm(&[("DB_URL", "postgres://localhost"), ("PORT", "nope")]),
    );

    case(
        "empty PORT",
        hm(&[("DB_URL", "postgres://localhost"), ("PORT", "   ")]),
    );

    case(
        "invalid DEBUG",
        hm(&[("DB_URL", "postgres://localhost"), ("DEBUG", "yes")]),
    );

    case(
        "empty DEBUG",
        hm(&[("DB_URL", "postgres://localhost"), ("DEBUG", "   ")]),
    );

    case(
        "invalid MAX_CONNECTIONS",
        hm(&[
            ("DB_URL", "postgres://localhost"),
            ("MAX_CONNECTIONS", "abc"),
        ]),
    );

    case(
        "empty MAX_CONNECTIONS",
        hm(&[
            ("DB_URL", "postgres://localhost"),
            ("MAX_CONNECTIONS", "   "),
        ]),
    );

    case(
        "invalid MODE",
        hm(&[("DB_URL", "postgres://localhost"), ("MODE", "weird")]),
    );
}

fn case(name: &str, map: HashMap<String, String>) {
    println!("\n--- {name} ---");

    match build_config(&map) {
        Ok(cfg) => {
            println!("OK:");
            println!("  port            = {}", cfg.port());
            println!("  debug           = {}", cfg.debug());
            println!("  db_url          = {}", cfg.db_url());
            println!("  max_connections = {}", cfg.max_connections());
            println!("  mode            = {:?}", cfg.mode());
        }
        Err(err) => {
            println!("ERR: {err:?}");
            explain_error(&err);
        }
    }
}

fn explain_error(err: &ConfigError) {
    match err {
        ConfigError::MissingKey { key } => println!("→ missing required key `{key}`"),

        ConfigError::EmptyValue { key } => println!("→ empty value for `{key}`"),

        ConfigError::InvalidU16 { key, raw } => println!("→ invalid u16 `{key}` = `{raw}`"),

        ConfigError::InvalidU32 { key, raw } => println!("→ invalid u32 `{key}` = `{raw}`"),

        ConfigError::InvalidBool { key, raw } => println!("→ invalid bool `{key}` = `{raw}`"),

        ConfigError::InvalidMode { key, raw } => println!("→ invalid mode `{key}` = `{raw}`"),
    }
}

fn hm(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}
