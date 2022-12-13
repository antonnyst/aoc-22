use std::{fs, cmp::Ordering};
use bstr::ByteSlice;
use serde_json::{Value, json};

pub fn d13() -> (String, String) {
    let read = fs::read("inputs/d13").unwrap();
    
    let mut start = 0;
    let mut lines: Vec<&[u8]> = vec![];
    for i in 0..read.len() {
        if read[i] == b'\n' {
            // End of line
            if start < i {
                lines.push(&read[start..i]);
            }
            start = i+1;
        }
    }

    let mut sum = 0;
    lines.chunks_exact(2).enumerate().for_each(|(i,x)|{
        let a: Value = serde_json::from_str(x[0].to_str().unwrap()).unwrap();
        let b: Value = serde_json::from_str(x[1].to_str().unwrap()).unwrap();
        let result = compare(&a, &b);
        match result {
            CheckResult::Correct => { sum += i+1; },
            CheckResult::Wrong => {}
            CheckResult::Continue => unimplemented!()
        }
    });

    lines.push(&[b'[',b'[',b'2',b']',b']']);
    lines.push(&[b'[',b'[',b'6',b']',b']']);

    lines.sort_by(|a,b| {
        let a: Value = serde_json::from_str(a.to_str().unwrap()).unwrap();
        let b: Value = serde_json::from_str(b.to_str().unwrap()).unwrap();
        let result = compare(&a, &b);
        match result {
            CheckResult::Correct => Ordering::Less,
            CheckResult::Wrong => Ordering::Greater,
            CheckResult::Continue => Ordering::Equal
        }
    });

    let mut part2 = 1;
    lines.iter().enumerate().for_each(|(i,x)|{
        if x.eq(&[b'[',b'[',b'2',b']',b']']) ||
           x.eq(&[b'[',b'[',b'6',b']',b']']) {
            part2 *= i+1
        }
    });

    (sum.to_string(), part2.to_string())
}

#[derive(PartialEq, Debug)]
enum CheckResult {
    Correct,
    Wrong,
    Continue
}

fn compare(a: &Value, b: &Value) -> CheckResult {
    let a_type = a.is_array();
    let b_type = b.is_array();
    match a_type {
        false => {
            match b_type {
                false => {
                    // Two integers
                    let a_val = a.as_u64().unwrap();
                    let b_val = b.as_u64().unwrap();
                    match a_val.cmp(&b_val) {
                        Ordering::Equal => {
                            return CheckResult::Continue;
                        },
                        Ordering::Less => {
                            return CheckResult::Correct;
                        },
                        Ordering::Greater => {
                            return CheckResult::Wrong;
                        }
                    }
                },
                true => {
                    // One list and integer
                    // Convert integer to list
                    let new_a = json!([a]);
                    return compare(&new_a,b);
                }
            }
        }
        true => {
            match b_type {
                false => {
                    // One list and integer
                    // Convert integer to list
                    let new_b = json!([b]);
                    return compare(a,&new_b);
                },
                true => {
                    // Two lists
                    let a_val = a.as_array().unwrap();
                    let b_val = b.as_array().unwrap();
                    
                    for i in 0..a_val.len() {
                        if b_val.len() == i {
                            return CheckResult::Wrong;
                        }
                        let result = compare(&a_val[i], &b_val[i]);
                        if !(CheckResult::Continue == result) {
                            return result;
                        }
                    }
                    if a_val.len() == b_val.len() {
                        return CheckResult::Continue;
                    }
                    return CheckResult::Correct;
                }
            }
        }
    };
}
