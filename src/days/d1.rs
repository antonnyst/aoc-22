use std::fs;

pub fn d1() -> (String, String) {
    let bytes = fs::read("inputs/d1").unwrap();

    let mut top_elves = vec![0,0,0];
    
    let mut elf_sum = 0;
    let mut start = 0;

    unsafe {
        for i in 0..bytes.len() {
            if bytes[i] == b'\n' {
                if start == i {
                    // Empty line
                    for i in 0..3 {
                        if elf_sum > top_elves[i] {
                            let swp = top_elves[i];
                            top_elves[i] = elf_sum;
                            elf_sum = swp;
                        }
                    }
                    elf_sum = 0;
                } else {
                    // End of number
                    elf_sum += std::str::from_utf8_unchecked(&bytes[start..i]).parse::<u32>().unwrap_unchecked();
                }
                start = i + 1;
            }
        }
    }

    let a = top_elves[0];
    let b:u32 = top_elves.iter().sum();

    (a.to_string(),b.to_string())
}

#[cfg(test)]
mod tests {
    use super::d1;
    #[test]
    fn main() {
        assert_eq!(d1(), (String::from("72511"),String::from("212117")));
    }
}