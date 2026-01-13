use day22::cli::args::{Args, Command, ParseOutcome};
use day22::cli::error::UsageError;
use day22::domain::types::SortSpec;

#[test]
fn smoke_arg_parse_print_basic() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
    ];
    
    let result = Args::parse(argv).expect("should parse");
    
    match result {
        ParseOutcome::Args(args) => {
            assert!(matches!(args.cmd, Command::Print));
            assert_eq!(args.file.as_ref().unwrap().to_str().unwrap(), "/tmp/test.txt");
            assert_eq!(args.sort, SortSpec::NameAsc);
        }
        ParseOutcome::Help => panic!("expected Args, got Help"),
    }
}

#[test]
fn smoke_arg_parse_print_with_sort() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--sort".to_string(),
        "price".to_string(),
    ];
    
    let result = Args::parse(argv).expect("should parse");
    
    match result {
        ParseOutcome::Args(args) => {
            assert!(matches!(args.cmd, Command::Print));
            assert_eq!(args.sort, SortSpec::PriceDescNameAsc);
        }
        ParseOutcome::Help => panic!("expected Args, got Help"),
    }
}

#[test]
fn smoke_arg_parse_add_basic() {
    let argv = vec![
        "day22".to_string(),
        "add".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--id".to_string(),
        "1".to_string(),
        "--sku".to_string(),
        "SKU1".to_string(),
        "--name".to_string(),
        "Apple".to_string(),
        "--price".to_string(),
        "100".to_string(),
    ];
    
    let result = Args::parse(argv).expect("should parse");
    
    match result {
        ParseOutcome::Args(args) => {
            match args.cmd {
                Command::Add { id, sku, name, price } => {
                    assert_eq!(id, "1");
                    assert_eq!(sku, "SKU1");
                    assert_eq!(name, "Apple");
                    assert_eq!(price, "100");
                }
                _ => panic!("expected Add command"),
            }
            assert_eq!(args.file.as_ref().unwrap().to_str().unwrap(), "/tmp/test.txt");
        }
        ParseOutcome::Help => panic!("expected Args, got Help"),
    }
}

#[test]
fn smoke_arg_parse_add_different_order() {
    let argv = vec![
        "day22".to_string(),
        "add".to_string(),
        "--sku".to_string(),
        "TEST".to_string(),
        "--price".to_string(),
        "500".to_string(),
        "--id".to_string(),
        "99".to_string(),
        "--file".to_string(),
        "/tmp/inv.txt".to_string(),
        "--name".to_string(),
        "Product".to_string(),
    ];
    
    let result = Args::parse(argv).expect("should parse");
    
    match result {
        ParseOutcome::Args(args) => {
            match args.cmd {
                Command::Add { id, sku, name, price } => {
                    assert_eq!(id, "99");
                    assert_eq!(sku, "TEST");
                    assert_eq!(name, "Product");
                    assert_eq!(price, "500");
                }
                _ => panic!("expected Add command"),
            }
        }
        ParseOutcome::Help => panic!("expected Args, got Help"),
    }
}

#[test]
fn smoke_arg_help_flag() {
    let argv = vec![
        "day22".to_string(),
        "--help".to_string(),
    ];
    
    let result = Args::parse(argv).expect("should parse");
    
    match result {
        ParseOutcome::Help => {}, // Expected
        ParseOutcome::Args(_) => panic!("expected Help, got Args"),
    }
}

#[test]
fn smoke_arg_help_short_flag() {
    let argv = vec![
        "day22".to_string(),
        "-h".to_string(),
    ];
    
    let result = Args::parse(argv).expect("should parse");
    assert!(matches!(result, ParseOutcome::Help));
}

#[test]
fn smoke_arg_unknown_command() {
    let argv = vec![
        "day22".to_string(),
        "delete".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    assert!(matches!(err, UsageError::UnknownCommand { .. }));
}

#[test]
fn smoke_arg_missing_command() {
    let argv = vec![
        "day22".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UsageError::MissingCommand { .. }));
}

#[test]
fn smoke_arg_print_missing_file() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UsageError::MissingRequiredFlag { .. }));
}

#[test]
fn smoke_arg_add_missing_id() {
    let argv = vec![
        "day22".to_string(),
        "add".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--sku".to_string(),
        "SKU1".to_string(),
        "--name".to_string(),
        "Apple".to_string(),
        "--price".to_string(),
        "100".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        UsageError::MissingRequiredFlag { flag } => {
            assert_eq!(flag, "--id");
        }
        _ => panic!("expected MissingRequiredFlag for --id, got {:?}", err),
    }
}

#[test]
fn smoke_arg_add_missing_sku() {
    let argv = vec![
        "day22".to_string(),
        "add".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--id".to_string(),
        "1".to_string(),
        "--name".to_string(),
        "Apple".to_string(),
        "--price".to_string(),
        "100".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        UsageError::MissingRequiredFlag { flag } => {
            assert_eq!(flag, "--sku");
        }
        _ => panic!("expected MissingRequiredFlag for --sku, got {:?}", err),
    }
}

#[test]
fn smoke_arg_add_missing_name() {
    let argv = vec![
        "day22".to_string(),
        "add".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--id".to_string(),
        "1".to_string(),
        "--sku".to_string(),
        "SKU1".to_string(),
        "--price".to_string(),
        "100".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        UsageError::MissingRequiredFlag { flag } => {
            assert_eq!(flag, "--name");
        }
        _ => panic!("expected MissingRequiredFlag for --name, got {:?}", err),
    }
}

#[test]
fn smoke_arg_add_missing_price() {
    let argv = vec![
        "day22".to_string(),
        "add".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--id".to_string(),
        "1".to_string(),
        "--sku".to_string(),
        "SKU1".to_string(),
        "--name".to_string(),
        "Apple".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        UsageError::MissingRequiredFlag { flag } => {
            assert_eq!(flag, "--price");
        }
        _ => panic!("expected MissingRequiredFlag for --price, got {:?}", err),
    }
}

#[test]
fn smoke_arg_unknown_flag() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--unknown".to_string(),
        "value".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UsageError::UnknownFlag { .. }));
}

#[test]
fn smoke_arg_empty_file_path() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
        "--file".to_string(),
        "".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UsageError::EmptyFilePath { .. }));
}

#[test]
fn smoke_arg_invalid_sort() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "--sort".to_string(),
        "invalid".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UsageError::InvalidSort { .. }));
}

#[test]
fn smoke_arg_unexpected_arg() {
    let argv = vec![
        "day22".to_string(),
        "print".to_string(),
        "--file".to_string(),
        "/tmp/test.txt".to_string(),
        "unexpected".to_string(),
    ];
    
    let result = Args::parse(argv);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UsageError::UnexpectedArg { .. }));
}
