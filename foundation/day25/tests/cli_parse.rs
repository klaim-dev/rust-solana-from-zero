use day25::app::service::AppCommand;
use day25::cli::{parse_args, UsageError};

fn args(input: &[&str]) -> Vec<String> {
    input.iter().map(|value| value.to_string()).collect()
}

#[test]
fn add_order_ok() {
    let cli = parse_args(args(&["add-order", "--id", "1", "--customer", "alice"]))
        .expect("parse args");
    let (cmd, file) = cli.into_app_command().expect("into app");
    assert!(file.is_none());

    match cmd {
        AppCommand::AddOrder { id, customer } => {
            assert_eq!(id.get(), 1);
            assert_eq!(customer, "alice");
        }
        _ => panic!("unexpected command"),
    }
}

#[test]
fn list_ok() {
    let cli = parse_args(args(&["list"])).expect("parse args");
    let (cmd, file) = cli.into_app_command().expect("into app");
    assert!(file.is_none());

    match cmd {
        AppCommand::List { customer } => assert!(customer.is_none()),
        _ => panic!("unexpected command"),
    }
}

#[test]
fn list_customer_ok() {
    let cli =
        parse_args(args(&["list", "--customer", "alice"])).expect("parse args");
    let (cmd, _file) = cli.into_app_command().expect("into app");

    match cmd {
        AppCommand::List { customer } => {
            assert_eq!(customer, Some("alice".to_string()));
        }
        _ => panic!("unexpected command"),
    }
}

#[test]
fn list_with_file_override() {
    let cli = parse_args(args(&["list", "--file", "/tmp/a"])).expect("parse args");
    let (cmd, file) = cli.into_app_command().expect("into app");
    assert_eq!(file, Some("/tmp/a".into()));

    match cmd {
        AppCommand::List { customer } => assert!(customer.is_none()),
        _ => panic!("unexpected command"),
    }
}

#[test]
fn help_flag_requested() {
    match parse_args(args(&["--help"])) {
        Err(UsageError::HelpRequested) => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn help_short_flag_requested() {
    match parse_args(args(&["-h"])) {
        Err(UsageError::HelpRequested) => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn missing_command() {
    match parse_args(args(&[])) {
        Err(UsageError::MissingCommand) => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn missing_flag_value() {
    match parse_args(args(&["add-order", "--id"])) {
        Err(UsageError::MissingFlagValue(flag)) if flag == "--id" => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn add_order_missing_customer() {
    match parse_args(args(&["add-order", "--id", "1"])) {
        Err(UsageError::MissingRequiredFlag("--customer")) => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn add_item_invalid_qty_int() {
    let result = parse_args(args(&[
        "add-item",
        "--id",
        "1",
        "--sku",
        "a",
        "--qty",
        "x",
    ]));
    match result {
        Err(UsageError::InvalidInt { flag: "--qty", input }) if input == "x" => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn list_id_is_unexpected() {
    match parse_args(args(&["list", "--id", "1"])) {
        Err(UsageError::UnexpectedFlagForCommand { cmd, flag: "--id" })
            if cmd == "list" => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn unknown_command() {
    match parse_args(args(&["foo", "--id", "1"])) {
        Err(UsageError::UnknownCommand(cmd)) if cmd == "foo" => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn add_order_invalid_id_value() {
    let cli =
        parse_args(args(&["add-order", "--id", "0", "--customer", "a"]))
            .expect("parse args");
    match cli.into_app_command() {
        Err(UsageError::InvalidValue { flag: "--id", input, .. })
            if input == "0" => {}
        _ => panic!("unexpected result"),
    }
}

#[test]
fn add_item_invalid_sku_value() {
    let cli = parse_args(args(&[
        "add-item",
        "--id",
        "1",
        "--sku",
        "",
        "--qty",
        "1",
    ]))
    .expect("parse args");
    match cli.into_app_command() {
        Err(UsageError::InvalidValue { flag: "--sku", input, .. })
            if input.is_empty() => {}
        _ => panic!("unexpected result"),
    }
}
