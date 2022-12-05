use std::fs;

pub fn d5() -> (String, String) {
    let read = fs::read("inputs/d5").unwrap();

    let mut start = 0;
    let mut lines: Vec<&[u8]> = vec![];
    for i in 0..read.len() {
        if read[i] == b'\n' {
            // End of line
            lines.push(&read[start..i]);
            start = i+1;
        }
    }

    let mut columns: Vec<Vec<char>> = vec![vec![]; 9];
    for i in 0..8 {
        let line = lines[i];
        for x in 0..9 {
            if line[1+x*4] != b' ' {
                columns[x].push(
                    line[1+x*4] as char
                );
            }
        }
    }

    for col in columns.iter_mut() {
        col.reverse();
    }
    
    let mut columns_2 = columns.clone();

    for i in 10..lines.len() {
        let line:Vec<&str> = std::str::from_utf8(lines[i]).unwrap().split(' ').collect();

        let count = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        // Part 1
        for _ in 0..count {
            let swp = columns[from].pop().unwrap();
            columns[to].push(swp);
        }

        let mut stack = vec![];
        for _ in 0..count {
            stack.push(columns_2[from].pop().unwrap());
        }
        stack.reverse();
        for c in stack {
            columns_2[to].push(c);
        }
    }
    
    let mut res1 = String::new();
    for col in columns.iter_mut() {
        res1.push(col.pop().unwrap_or(' '));
    }

    let mut res2 = String::new();
    for col in columns_2.iter_mut() {
        res2.push(col.pop().unwrap_or(' '));
    }

    (res1, res2)
}

#[cfg(test)]
mod tests {
    use super::d5;
    #[test]
    fn main() {
        assert_eq!(d5(), (String::from("FWNSHLDNZ"),String::from("RNRGDNFQG")));
    }
}