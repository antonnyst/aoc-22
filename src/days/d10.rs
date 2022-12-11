use std::fs;

pub fn d10() -> (String, String) {
    let read = fs::read_to_string("inputs/d10").unwrap();
    let lines = read.split("\n").collect::<Vec<&str>>();

    let points = vec![20,60,100,140,180,220];
    
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;

    let mut pixels = vec!['.'; 40*6];

    for line in lines {
        // Parse command
        let split = line.split_once(" ").unwrap_or(("noop", ""));
        match split.0 {
            "noop" => {
                // Start of cycle
                cycle += 1;

                // During cycle
                if points.contains(&cycle) {
                    sum += x * cycle;
                }
                if (x - ((cycle-1)%40) as i32).abs() < 2 {
                    // Draw pixel
                    pixels[(cycle-1) as usize] = '#';
                }

                // After cycle

                
            },
            "addx" => {
                // Start of cycle 1
                cycle += 1;

                // During cycle 1
                if points.contains(&cycle) {
                    sum += x * cycle;
                }
                if (x - ((cycle-1)%40) as i32).abs() < 2 {
                    // Draw pixel
                    pixels[(cycle-1) as usize] = '#';
                }


                // After cycle 1
                
                // Start of cycle 2
                cycle += 1;

                // During cycle 2
                if points.contains(&cycle) {
                    sum += x * cycle;
                }
                if (x - ((cycle-1)%40) as i32).abs() < 2 {
                    // Draw pixel
                    pixels[(cycle-1) as usize] = '#';
                }

                // After cycle 2
                x += split.1.parse::<i32>().unwrap();
            } 
            _ => unimplemented!()
        }

        
    }

    let image: String = pixels.iter().enumerate().flat_map(|(i,x)| {
        if i % 40 == 0 {
            return [&'\n', x]
        } else {
            return [&' ', x]
        }
    }).collect::<String>();

    (sum.to_string(), image)
}