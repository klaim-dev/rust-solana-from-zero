use super::*;
use crate::env::FakeEnv;
use std::io::Write;
use std::io::ErrorKind;
use std::path::PathBuf;

#[test]
fn parse_valid_toml() {
    let mut f = tempfile::NamedTempFile::new().unwrap();

    let content = r#"
app_env = "dev"
data_file = "/tmp/data.json"
page_size = 50
"#;

    f.write_all(content.as_bytes()).unwrap();

    let path: PathBuf = f.path().to_path_buf();
    let cfg = load_from_file(&path).unwrap();

    assert_eq!(cfg.app_env.as_deref(), Some("dev"));
    assert_eq!(
        cfg.data_file.as_ref(),
        Some(&PathBuf::from("/tmp/data.json"))
    );
    assert_eq!(cfg.page_size, Some(50));
}

#[test]
fn parse_invalid_toml() {
    let mut f = tempfile::NamedTempFile::new().unwrap();

    let content = r#"
app_env = "dev
page_size = 50
"#;

    f.write_all(content.as_bytes()).unwrap();

    let path: PathBuf = f.path().to_path_buf();
    let err = load_from_file(&path).unwrap_err();

    match err {
        ConfigError::BadToml { path: p } => assert_eq!(p, path),
        _ => panic!("Expected BadToml error, got {:?}", err),
    }
}

#[test]
fn env_usize_ok() {
    let env = FakeEnv::new(&[("DB_URL", "42")]);
    let res = env_usize(&env, "DB_URL");
    assert_eq!(res, Ok(Some(42)))
}

#[test]
fn env_usize_missing() {
    let env = FakeEnv::default();
    let res = env_usize(&env, "DB_URL");
    assert_eq!(res, Ok(None))
}

#[test]
fn env_usize_empty() {
    let env = FakeEnv::new(&[("DB_URL", "")]);
    let err = env_usize(&env, "DB_URL").unwrap_err();
    assert_eq!(err, ConfigError::EmptyEnv { key: "DB_URL" });
}

#[test]
fn env_usize_invalid() {
    let env = FakeEnv::new(&[("DB_URL", "abc")]);
    let err = env_usize(&env, "DB_URL").unwrap_err();
    assert_eq!(
        err,
        ConfigError::InvalidEnv {
            key: "DB_URL",
            reason: "expected usize"
        }
    )
}

#[test]
fn missing() {
    let env = FakeEnv::default();
    let err = require_env(&env, "DB_URL").unwrap_err();
    assert_eq!(err, ConfigError::MissingRequired { key: "DB_URL" });
}

#[test]
fn empty() {
    let env = FakeEnv::new(&[("DB_URL", "")]);
    let err = require_env(&env, "DB_URL").unwrap_err();
    assert_eq!(err, ConfigError::EmptyEnv { key: "DB_URL" });
}

#[test]
fn ok() {
    let env = FakeEnv::new(&[("DB_URL", "postgres://fake")]);
    let v = require_env(&env, "DB_URL").unwrap();
    assert_eq!(v, "postgres://fake");
}

#[test]
fn config_load_file_not_found() {
    let env = FakeEnv::new(&[("DATA_FILE", "/tmp/data.json")]);
    let dir = tempfile::tempdir().unwrap();
    let nonexistent_path = dir.path().join("config.toml");

    let cfg = Config::load(&env, &nonexistent_path).unwrap();

    assert_eq!(cfg.data_file(), std::path::Path::new("/tmp/data.json"));
    assert_eq!(cfg.app_env(), &AppEnv::Dev);
    assert_eq!(cfg.page_size(), 50);
}

#[test]
fn load_from_file_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("file.toml");
    let err = load_from_file(&path).unwrap_err();

    match err {
        ConfigError::Io {
            kind: ErrorKind::NotFound,
            ..
        } => {}
        other => panic!("Expected NotFound error, got {:?}", other),
    }
}

#[test]
fn load_missing_required() {
    let env = FakeEnv::default();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.toml");

    let err = Config::load(&env, &path).unwrap_err();
    assert_eq!(err, ConfigError::MissingRequired { key: "DATA_FILE" });
}

#[test]
fn load_bad_toml() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    let content = r#"
app_env = "dev
page_size = 50
"#;
    f.write_all(content.as_bytes()).unwrap();

    let env = FakeEnv::default();
    let path = f.path().to_path_buf();
    let err = Config::load(&env, &path).unwrap_err();

    match err {
        ConfigError::BadToml { path: p } => assert_eq!(p, path),
        other => panic!("Expected BadToml error, got {:?}", other),
    }
}

#[test]
fn load_bad_page_size() {
    let env = FakeEnv::new(&[("DATA_FILE", "/tmp/x"), ("PAGE_SIZE", "abc")]);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.toml");

    let err = Config::load(&env, &path).unwrap_err();
    assert_eq!(
        err,
        ConfigError::InvalidEnv {
            key: "PAGE_SIZE",
            reason: "expected usize"
        }
    );
}

#[test]
fn load_env_overrides_file() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    let content = r#"
data_file = "/tmp/a"
page_size = 10
"#;
    f.write_all(content.as_bytes()).unwrap();

    let env = FakeEnv::new(&[("PAGE_SIZE", "50")]);
    let cfg = Config::load(&env, f.path()).unwrap();

    assert_eq!(cfg.data_file(), std::path::Path::new("/tmp/a"));
    assert_eq!(cfg.page_size(), 50);
}
