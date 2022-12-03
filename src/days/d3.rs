use std::{fs, vec};

pub fn d3() -> (String, String) {
    let read = fs::read("inputs/d3").unwrap();
    
    let mut sum_1: u32 = 0;
    let mut sum_2: u32 = 0;


    let mut start = 0;
    let mut lines: Vec<&[u8]> = vec![];
    for i in 0..read.len() {
        if read[i] == b'\n' {
            // End of line
            lines.push(&read[start..i]);
            start = i+1;
        }
    }

    for i in 0..lines.len() {
        let line = lines[i];
        let first = &line[..line.len()/2];
        let second = &line[line.len()/2..];

        for c in first {
            if second.contains(c) {
                sum_1 += priority(c) as u32;
                break;
            }
        }

        if i % 3 == 0 {
            // Check the group
            let one = lines[i];
            let two = lines[i+1];
            let three = lines[i+2];
            for c in one {
                if two.contains(c) && three.contains(c) {
                    sum_2 += priority(c) as u32;
                    break;
                }
            }
        }
    }

    (sum_1.to_string(), sum_2.to_string())
}

fn priority(c: &u8) -> u8 {
    if c < &97 {
        return c - 38;
    } else {
        return c - 96;
    }
}

#[cfg(test)]
mod tests {
    use super::d3;
    #[test]
    fn main() {
        assert_eq!(d3(), (String::from("7821"),String::from("2752")));
    }
}