use std::fs;

pub fn d4() -> (String, String) {
    let read = fs::read("inputs/d4").unwrap();

    let mut contains = 0;
    let mut overlaps = 0;

    let mut nums:Vec<u32> = vec![];
    let mut start = 0;
    unsafe {
        for i in 0..read.len() {
            match read[i] {
                b','|b'-' => {
                    let num = std::str::from_utf8_unchecked(&read[start..i]).parse::<u32>().unwrap_unchecked();
                    nums.push(num);
                    start = i + 1;
                },
                b'\n' => {
                    let num = std::str::from_utf8_unchecked(&read[start..i]).parse::<u32>().unwrap_unchecked();
                    nums.push(num);
                    start = i + 1;

                    if (nums[0] >= nums[2] && nums[1] <= nums[3]) ||
                       (nums[2] >= nums[0] && nums[3] <= nums[1]) {
                        contains += 1;
                        overlaps += 1;
                    } else if 
                        (nums[0] >= nums[2] && nums[0] <= nums[3]) || 
                        (nums[1] >= nums[2] && nums[1] <= nums[3]) {
                        overlaps += 1;
                    }

                    nums.clear();
                }
                _ => {}
            }

        }


    }

    (contains.to_string(), overlaps.to_string())
}

#[cfg(test)]
mod tests {
    use super::d4;
    #[test]
    fn main() {
        assert_eq!(d4(), (String::from("644"),String::from("926")));
    }
}