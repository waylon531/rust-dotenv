extern crate dotenv;

use dotenv::*;
use std::env;
use std::path::Path;
use std::fs;

fn init() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR").to_string() + "/tests";
    let new_path = Path::new(&manifest_dir);
    assert!(env::set_current_dir(&new_path).is_ok());
}

fn clear() {
    env::set_var("TESTKEY", "");
}

#[test]
fn test_dotenv() {
    init();
    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");
    clear();
}

#[test]
fn test_dotenv_child_dir() {
    init();
    let new_path = Path::new("./child");
    assert!(env::set_current_dir(&new_path).is_ok());
    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");
    clear();
}

#[test]
fn test_from_filename() {
    init();
    from_filename("notenv").ok();
    assert_eq!(env::var("NOTTESTKEY").unwrap(), "not_test_val");
    //assert!(!env::var("TESTKEY").is_ok());
    println!("ff {:?}", env::var("TESTKEY"));
    clear();
}
