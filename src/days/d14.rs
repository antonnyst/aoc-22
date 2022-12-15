use std::fs;

pub fn d14() -> (String, String) {
    let read = fs::read_to_string("inputs/d14").unwrap();
    let mut lines = read.split("\n").collect::<Vec<&str>>();
    lines.remove(lines.len()-1);

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    let mut splits = vec![];


    for line in lines {
        let split: Vec<(usize,usize)> = line.split(" -> ")
            .map(|x| {
                let (a,b) = x.split_once(',')
                .unwrap();

                (a.parse().unwrap(),b.parse().unwrap())
            }   
            ).collect();
        
        split.iter().for_each(|x| {
            let a = x.0;
            let b = x.1;

            min_x = min_x.min(a);
            max_x = max_x.max(a);
            min_y = min_y.min(b);
            max_y = max_y.max(b);
        });
        splits.push(split);
    }

    let x_offset = min_x;
    let width = min_x.abs_diff(max_x) + 1;
    let height = max_y+1;

    let mut map = vec!['.'; width*height];

    for split in splits.iter() {
        for i in 1..split.len() {
            if split[i].0 != split[i-1].0 {
                // Difference in x
                let min = split[i].0.min(split[i-1].0);
                let max = split[i].0.max(split[i-1].0);
                for x in min..=max {
                    map[split[i].1 * width + (x - x_offset)] = '#';
                }
            } else {
                // Difference in y
                let min = split[i].1.min(split[i-1].1);
                let max = split[i].1.max(split[i-1].1);
                for y in min..=max {
                    map[y * width + (split[i].0 - x_offset)] = '#';
                }
            }

        }
    }

    /*for i in 0..map.len() {
        if i % width == 0 {
            println!();
        }
        print!("{}",map[i]);
    }*/

    let mut count_1 = 0;
    while !step(&mut map, width, height, x_offset) {
        count_1 += 1;
    }

    // Part 2
    let height = max_y+1+2;
    let x_offset = min_x - height;
    let width = min_x.abs_diff(max_x) + 1 + height*2;

    let mut map = vec!['.'; width*height];

    for x in 0..width {
        map[(height-1) * width + (x)] = '#';
    }

    for split in splits.iter() {
        for i in 1..split.len() {
            if split[i].0 != split[i-1].0 {
                // Difference in x
                let min = split[i].0.min(split[i-1].0);
                let max = split[i].0.max(split[i-1].0);
                for x in min..=max {
                    map[split[i].1 * width + (x - x_offset)] = '#';
                }
            } else {
                // Difference in y
                let min = split[i].1.min(split[i-1].1);
                let max = split[i].1.max(split[i-1].1);
                for y in min..=max {
                    map[y * width + (split[i].0 - x_offset)] = '#';
                }
            }

        }
    }
    /*
    for i in 0..map.len() {
        if i % width == 0 {
            println!();
        }
        print!("{}",map[i]);
    }*/

    let mut count_2 = 0;
    while !step(&mut map, width, height, x_offset) {
        count_2 += 1;
    }

    (count_1.to_string(), count_2.to_string())
}

fn step(map: &mut Vec<char>, width: usize, height: usize, x_offset: usize) -> bool {
    let mut s_pos = (500,0);

    if map[(s_pos.1) * width + (s_pos.0-x_offset)] != '.' {
        return true;
    }
    loop {
        // Check if we are at bottom row
        if s_pos.1 == height-1 {
            // fall out of map
            break true;
        }

        // Check below
        if map[(s_pos.1+1) * width + (s_pos.0-x_offset)] == '.' {
            // Empty, lets go
            s_pos = (s_pos.0,s_pos.1+1);
            continue;
        }

        // Check bottom left
        if map[(s_pos.1+1) * width + (s_pos.0-1-x_offset)] == '.' {
            // Empty, lets go
            s_pos = (s_pos.0-1,s_pos.1+1);
            continue;
        }

        // Check bottom right
        if map[(s_pos.1+1) * width + (s_pos.0+1-x_offset)] == '.' {
            // Empty, lets go
            s_pos = (s_pos.0+1,s_pos.1+1);
            continue;
        }

        // We cannot move
        map[(s_pos.1) * width + (s_pos.0-x_offset)] = 'o';
        break false;
    }
}