use std::fs;

pub fn d19() -> (String, String) {
    let read = fs::read_to_string("inputs/d19").unwrap();
    let _lines = read.split("\n").collect::<Vec<&str>>();

    (String::new(), String::new())
}