use std::fs;

pub fn d6() -> (String, String) {
    let read = fs::read("inputs/d6").unwrap();    

    let mut index_1 = 0;
    for i in 3..read.len() {
        let slice = &read[i-3..=i];
        let mut works = true;
        for x in 0..4 {
            for y in 0..4 {
                if x == y {
                    continue;
                }
                if slice[x] == slice[y] {
                    works = false;
                    break;
                }
            }
        }
        if works {
            index_1 = i + 1;
            break;
        }
    }

    let mut index_2 = 0;
    for i in 13..read.len() {
        let slice = &read[i-13..=i];
        let mut works = true;
        for x in 0..14 {
            for y in 0..14 {
                if x == y {
                    continue;
                }
                if slice[x] == slice[y] {
                    works = false;
                    break;
                }
            }
        }
        if works {
            index_2 = i + 1;
            break;
        }
    }

    (index_1.to_string(), index_2.to_string())
}