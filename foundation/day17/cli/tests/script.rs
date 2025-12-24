use catalog::store::catalog::Catalog;
use cli::app::engine::execute;
use cli::app::view::Response;
use cli::cli::command::Command;
use cli::cli::render::render;
use std::str::FromStr;

pub fn run_script(lines: &[&str]) -> Vec<String> {
    let mut catalog = Catalog::new();
    let mut output = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let cmd = match Command::from_str(trimmed) {
            Ok(cmd) => cmd,
            Err(e) => {
                output.push(format!("ERR PARSE {}", e));
                continue;
            }
        };

        if matches!(cmd, Command::Exit) {
            let resp = execute(&mut catalog, cmd).unwrap();
            output.push(render(resp));
            break;
        }

        match execute(&mut catalog, cmd) {
            Ok(resp) => {
                if matches!(resp, Response::Exit) {
                    output.push(render(resp));
                    break;
                } else {
                    output.push(render(resp));
                }
            }
            Err(e) => {
                output.push(format!("ERR {}", e));
            }
        }
    }

    output
}

#[test]
fn test_script_full_workflow() {
    let lines = vec![
        "create sku=BOOK001 name=RustBook category=books price=2999 active=true",
        "create sku=BOOK002 name=PythonGuide category=books price=1999 active=true",
        "list",
        "get id=1",
        "update id=1 name=UpdatedRustBook price=3499",
        "delete id=2",
        "list",
        "exit",
    ];

    let output = run_script(&lines);

    // Output: 2 creates, 1 list, 1 get, 1 update, 1 delete, 1 list, 1 exit = 8 total
    assert_eq!(output.len(), 8);

    // Check create responses
    assert!(output[0].starts_with("OK CREATED"));
    assert!(output[1].starts_with("OK CREATED"));

    // Check list response (should have 2 items)
    // SKU is normalized to lowercase in the catalog
    let list_output = &output[2];
    assert!(list_output.contains("ITEM"));
    assert!(list_output.contains("book001")); // SKU normalized
    assert!(list_output.contains("book002")); // SKU normalized

    // Check get response
    assert!(output[3].contains("ITEM"));
    assert!(output[3].contains("book001")); // SKU normalized
    assert!(output[3].contains("RustBook"));

    // Check update response
    assert!(output[4].starts_with("OK UPDATE"));

    // Check delete response
    assert!(output[5].starts_with("OK DELETED"));

    // Check final list (should have 1 item)
    let final_list = &output[6];
    assert!(final_list.contains("ITEM"));
    assert!(final_list.contains("book001")); // SKU normalized
    assert!(!final_list.contains("book002"));

    // Check exit
    assert_eq!(output[7], "OK BYE");
}

#[test]
fn test_script_with_sku_get() {
    let lines = vec![
        "create sku=TEST001 name=TestItem category=electronics price=1000 active=true",
        "get sku=TEST001",
        "exit",
    ];

    let output = run_script(&lines);

    // Debug output if needed
    // for (i, line) in output.iter().enumerate() {
    //     eprintln!("[{}] {}", i, line);
    // }

    assert_eq!(output.len(), 3);
    assert!(output[0].starts_with("OK CREATED"));
    // ITEM format: "ITEM id= {} | sku= {} | name= \"{}\" | category= {} | price= {} | active= {}"
    assert!(output[1].starts_with("ITEM"));
    assert!(output[1].contains("test001")); // SKU is normalized to lowercase
    assert!(output[1].contains("TestItem"));
    assert_eq!(output[2], "OK BYE");
}

#[test]
fn test_script_negative_cases() {
    // Test duplicate SKU
    let lines = vec![
        "create sku=DUPLICATE name=First category=books price=1000 active=true",
        "create sku=DUPLICATE name=Second category=books price=2000 active=true",
        "exit",
    ];
    let output = run_script(&lines);
    assert_eq!(output.len(), 3);
    assert!(output[0].starts_with("OK CREATED"));
    assert!(output[1].starts_with("ERR"));
    assert!(output[1].contains("duplicate sku"));
    assert_eq!(output[2], "OK BYE");

    // Test update missing ID
    let lines = vec!["update id=999 name=Test", "exit"];
    let output = run_script(&lines);
    assert_eq!(output.len(), 2);
    assert!(output[0].starts_with("ERR"));
    assert!(output[0].contains("item not found"));
    assert_eq!(output[1], "OK BYE");

    // Test delete unknown ID
    let lines = vec!["delete id=999", "exit"];
    let output = run_script(&lines);
    assert_eq!(output.len(), 2);
    assert!(output[0].starts_with("ERR"));
    assert!(output[0].contains("item not found"));
    assert_eq!(output[1], "OK BYE");
}
