use std::{fs, collections::HashSet};

pub fn d9() -> (String, String) {
    let read = fs::read("inputs/d9").unwrap();
    let lines: Vec<&[u8]> = read.split(|&x| x==b'\n').collect();



    let mut map_1: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0,0);
    let mut tail = (0,0);

    for i in 0..lines.len()-1 {
        let dir = &lines[i][0];
        let count = std::str::from_utf8(&lines[i][2..]).unwrap().parse().unwrap();
        for _ in 0..count {
            match dir {
                b'U' => {
                    head.1 += 1;
                },
                b'R' => {
                    head.0 += 1;

                },
                b'D' => {
                    head.1 -= 1;

                },
                b'L' => {
                    head.0 -= 1;

                }
                _ => unimplemented!()
            }

            let dx = head.0 - tail.0;
            let dy = head.1 - tail.1;

            match (dx,dy) {
                (-2,0) => {
                    // left
                    tail.0 -= 1;
                }
                (2,0) => {
                    // right
                    tail.0 += 1;
                }
                (0,2) => {
                    // top
                    tail.1 += 1;
                }
                (0,-2) => {
                    // bottom
                    tail.1 -= 1;
                }
                (-2, -1)|(-1,-2) => {
                    // top left
                    tail.0 -= 1;
                    tail.1 -= 1;
                }
                (2, -1)|(1, -2) => {
                    // top right
                    tail.0 += 1;
                    tail.1 -= 1;
                }
                (-2, 1)|(-1, 2) => {
                    // bottom left
                    tail.0 -= 1;
                    tail.1 += 1;
                }
                (2, 1)|(1, 2) => {
                    // bottom right
                    tail.0 += 1;
                    tail.1 += 1;
                }
                _ => {}
            }

            map_1.insert(tail);
        }
    }

    let mut map_2: HashSet<(i32, i32)> = HashSet::new();
    let mut parts = vec![(0,0); 10];

    for i in 0..lines.len()-1 {
        let dir = &lines[i][0];
        let count = std::str::from_utf8(&lines[i][2..]).unwrap().parse().unwrap();
        for _ in 0..count {
            // Move head
            match dir {
                b'U' => {
                    parts[0].1 += 1;
                },
                b'R' => {
                    parts[0].0 += 1;

                },
                b'D' => {
                    parts[0].1 -= 1;

                },
                b'L' => {
                    parts[0].0 -= 1;

                }
                _ => unimplemented!()
            }

            // Move parts in reaction
            for p in 1..10 {
                let dx = parts[p-1].0 - parts[p].0;
                let dy = parts[p-1].1 - parts[p].1;
                match (dx,dy) {
                    (-2,0) => {
                        // left
                        parts[p].0 -= 1;
                    }
                    (2,0) => {
                        // right
                        parts[p].0 += 1;
                    }
                    (0,2) => {
                        // top
                        parts[p].1 += 1;
                    }
                    (0,-2) => {
                        // bottom
                        parts[p].1 -= 1;
                    }
                    (-2, -1)|(-1,-2)|(-2,-2) => {
                        // top left
                        parts[p].0 -= 1;
                        parts[p].1 -= 1;
                    }
                    (2, -1)|(1, -2)|(2,-2) => {
                        // top right
                        parts[p].0 += 1;
                        parts[p].1 -= 1;
                    }
                    (-2, 1)|(-1, 2)|(-2,2) => {
                        // bottom left
                        parts[p].0 -= 1;
                        parts[p].1 += 1;
                    }
                    (2, 1)|(1, 2)|(2,2) => {
                        // bottom right
                        parts[p].0 += 1;
                        parts[p].1 += 1;
                    }
                    _ => {}
                }
            } 

            map_2.insert(parts[parts.len()-1]);
        }
    }

    (map_1.iter().count().to_string(), map_2.iter().count().to_string())
}