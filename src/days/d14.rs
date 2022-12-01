use std::fs;

pub fn d14() -> (String, String) {
    let read = fs::read_to_string("inputs/d14").unwrap();
    let _lines = read.split("\n").collect::<Vec<&str>>();

    (String::new(), String::new())
}