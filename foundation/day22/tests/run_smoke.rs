use std::{fs, path::PathBuf};

use day22::app::run::run;
use day22::cli::args::{Args, Command};
use day22::domain::types::SortSpec;

#[test]
fn run_smoke_print_ok() {
    //1) temp file 
    let dir = std::env::temp_dir().join("day22_run_smoke");
    let _ = fs::create_dir_all(&dir);
    let path: PathBuf = dir.join("inventory.txt");
     
     //2) write text
    let text = "\
    id=1 sku=SKU1 name=Apple price=100
    id=2 sku=SKU2 name=Banana price=200
    ";
    fs::write(&path, text).expect("write temp inventory");

    //3) Args for run
    let args = Args{
        cmd: Command::Print,
        file: Some(path.clone()),
        sort: SortSpec::NameAsc,
    };

    //4) Call run
    let out = run(args).expect("run should succeed");

    //5) assert baseline
    assert!(out.starts_with("OK\n"), "out={out:?}");
    assert!(out.contains("ITEM"), "out={out:?}");

    let _ = fs::remove_file(&path);
    let _ = fs::remove_dir(&dir);

}