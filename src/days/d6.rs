use std::fs;

pub fn d6() -> (String, String) {
    let read = fs::read("inputs/d6").unwrap();    

    let index_1 = moving_window(&read, 4);
    let index_2 = moving_window(&read, 14);

    (index_1.to_string(), index_2.to_string())
}

fn moving_window(data: &Vec<u8>, size: usize) -> usize {
    let mut seen = vec![0; 128];
    for i in 0..size {
        seen[data[i] as usize] += 1;
    }

    for i in size..data.len() {
        if !seen.iter().any(|x| x > &1) {
            return i;
        }

        seen[data[i] as usize] += 1;
        seen[data[i-size] as usize] -= 1;
    };
    return 0;
}