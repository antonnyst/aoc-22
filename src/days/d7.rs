use std::fs;

pub fn d7() -> (String, String) {
    let read = fs::read_to_string("inputs/d7").unwrap();
    let _lines = read.split("\n").collect::<Vec<&str>>();

    (String::new(), String::new())
}