use std::fs;

pub fn d15() -> (String, String) {
    let read = fs::read_to_string("inputs/d15").unwrap();
    let _lines = read.split("\n").collect::<Vec<&str>>();

    (String::new(), String::new())
}