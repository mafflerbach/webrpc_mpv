use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_fixture_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let v: Value = serde_json::from_str(contents.as_str()).unwrap();
    v.to_string()
}
