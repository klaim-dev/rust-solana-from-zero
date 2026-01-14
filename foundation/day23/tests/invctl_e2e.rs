use std::fs;
use std::process::Command;

#[test]
fn e2e_runs_without_file_uses_env() {
    let dir = tempfile::tempdir().unwrap();
    let data_path = dir.path().join("inventory.txt");
    let text = "id=1 sku=SKU1 name=Apple price=100\n";
    fs::write(&data_path, text).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_day23"))
        .arg("print")
        .current_dir(dir.path())
        .env_remove("DATA_FILE")
        .env_remove("PAGE_SIZE")
        .env_remove("APP_ENV")
        .env("DATA_FILE", &data_path)
        .output()
        .expect("run invctl");

    assert!(output.status.success(), "status={:?}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK\n"), "stdout={stdout:?}");
    assert!(stdout.contains("name=\"Apple\""), "stdout={stdout:?}");
}

#[test]
fn e2e_cli_file_without_env_or_config() {
    let dir = tempfile::tempdir().unwrap();
    let data_path = dir.path().join("inventory.txt");
    let text = "id=1 sku=SKU1 name=Apple price=100\n";
    fs::write(&data_path, text).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_day23"))
        .arg("print")
        .arg("--file")
        .arg(&data_path)
        .current_dir(dir.path())
        .env_remove("DATA_FILE")
        .env_remove("PAGE_SIZE")
        .env_remove("APP_ENV")
        .output()
        .expect("run invctl");

    assert!(output.status.success(), "status={:?}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("name=\"Apple\""), "stdout={stdout:?}");
}

#[test]
fn e2e_cli_file_overrides_config() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    let file_a = dir.path().join("a.txt");
    let file_b = dir.path().join("b.txt");

    fs::write(&file_a, "id=1 sku=SKU1 name=Apple price=100\n").unwrap();
    fs::write(&file_b, "id=2 sku=SKU2 name=Banana price=200\n").unwrap();

    let config = format!(
        "data_file = \"{}\"\npage_size = 10\n",
        file_a.display()
    );
    fs::write(&config_path, config).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_day23"))
        .arg("print")
        .arg("--file")
        .arg(&file_b)
        .current_dir(dir.path())
        .env_remove("DATA_FILE")
        .env_remove("PAGE_SIZE")
        .env_remove("APP_ENV")
        .output()
        .expect("run invctl");

    assert!(output.status.success(), "status={:?}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("name=\"Banana\""), "stdout={stdout:?}");
    assert!(!stdout.contains("name=\"Apple\""), "stdout={stdout:?}");
}
