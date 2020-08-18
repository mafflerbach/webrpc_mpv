use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_fixture_file<P: AsRef<Path>>(path: P) -> String {
    println!("{:?}", path.as_ref());
    let mut file = match File::open(path) {
        Err(e) => panic!("Caninot open file:{}  ", e),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let v: Value = serde_json::from_str(contents.as_str()).unwrap();
    v.to_string()
}

pub fn read_html_fixture_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.to_string()
}
