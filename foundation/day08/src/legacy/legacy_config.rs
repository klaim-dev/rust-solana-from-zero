#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub use_tls: bool,
    pub timeout_ms: i32,
}

pub fn apply_line_legacy(cfg: &mut Config, line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return true;
    }

    let parts: Vec<_> = trimmed.split('=').collect();
    if parts.len() != 2 {
        return false;
    }

    let key = parts[0].trim();
    let value = parts[1].trim();

    if key == "host" {
        cfg.host = value.to_string();
        true
    } else if key == "port" {
        let parsed = value.parse::<i32>().unwrap_or(-1);
        if parsed <= 0 {
            false
        } else {
            cfg.port = parsed;
            true
        }
    } else if key == "use_tls" {
        cfg.use_tls = value == "true";
        true
    } else if key == "timeout_ms" {
        let parsed = value.parse::<i32>().unwrap_or(0);
        if parsed < 0 {
            false
        } else {
            cfg.timeout_ms = parsed;
            true
        }
    } else {
        true
    }
}

pub fn load_config_legacy(cfg: &mut Config, input: &str) -> bool {
    let mut ok = true;
    for line in input.lines() {
        if !apply_line_legacy(cfg, line) {
            ok = false;
        }
    }
    ok
}
