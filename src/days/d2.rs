use std::fs;

pub fn d2() -> (String, String) {
    let bytes = fs::read("inputs/d2").unwrap();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..bytes.len()/4 {
        sum1 += score_1(bytes[i*4], bytes[i*4+2]);
        sum2 += score_2(bytes[i*4], bytes[i*4+2]);
    }

    (sum1.to_string(), sum2.to_string())
}

fn score_1(opponent: u8, own: u8) -> u32 {
    match (opponent, own) {
        (b'A', b'X') => 1 + 3,
        (b'A', b'Y') => 2 + 6,
        (b'A', b'Z') => 3 + 0,
        (b'B', b'X') => 1 + 0,
        (b'B', b'Y') => 2 + 3,
        (b'B', b'Z') => 3 + 6,
        (b'C', b'X') => 1 + 6,
        (b'C', b'Y') => 2 + 0,
        (b'C', b'Z') => 3 + 3,
        _ => panic!()
    }    
}

fn score_2(opponent: u8, own: u8) -> u32 {
    match (opponent, own) {
        (b'A', b'X') => 3 + 0,
        (b'A', b'Y') => 1 + 3,
        (b'A', b'Z') => 2 + 6,
        (b'B', b'X') => 1 + 0,
        (b'B', b'Y') => 2 + 3,
        (b'B', b'Z') => 3 + 6,
        (b'C', b'X') => 2 + 0,
        (b'C', b'Y') => 3 + 3,
        (b'C', b'Z') => 1 + 6,
        _ => panic!()
    }    
}

#[cfg(test)]
mod tests {
    use super::d2;
    #[test]
    fn main() {
        assert_eq!(d2(), (String::from("13009"),String::from("10398")));
    }
}